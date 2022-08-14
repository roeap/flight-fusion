//! # Authenticates Azure JWT tokens.
//!
//! This library will fetch public keys from Microsoft and use those keys to validate the
//! authenticity of a token you provide. It defaults to validating and mapping Azure Id tokens for
//! you out of the box, but should work with other tokens as well if you use a custom validator.
//!
//! It uses `request` with the "blocking" feature to fetch metadata and public
//! keys, but used correctly it will only update these once a day.
//!
//! # Dafault validation
//!
//! **There are mainly six conditions a well formed token will need to meet to be validated:**
//! 1. That the token is issued by Azure and is not tampered with
//! 2. That this token is issued for use in your application
//! 3. That the token is not expired
//! 4. That the token is not used before it's valid
//! 5. That the token is not issued in the future
//! 6. That the algorithm in the token header is the same as we use*
//!
//! * Note that we do NOT use the token header to set the algorithm for us, look [at this article
//! for more information on why that would be bad](https://auth0.com/blog/critical-vulnerabilities-in-json-web-token-libraries/)
//!
//! The validation will `Error` on a failed validation providing more granularity for library users
//! to find out why the token was rejected.
//!
//! If the token is invalid it will return an Error instead of a boolean. The main reason for this
//! is easier logging of what type of test it failed.
//!
//! You also have a `validate_custom` mathod which gives you full control over the mapping of the token
//! fields and more control over the validation.
//!
//! # Security
//! You will need a private app_id created by Azure for your application to be able to veriify that
//! the token is created for your application (and not anyone with a valid Azure token can log in)
//! and you will need to authenticate that the user has the right access to your system.
//!
//! For more information, see this artice: https://docs.microsoft.com/en-us/azure/active-directory/develop/id-tokens
//!
//! # Example in webserver
//!
//! ```rust, ignore
//! struct AppState {
//!     azure_auth: auth::AzureAuth,
//! }
//!
//! pub fn start_web_server(port: &str) -> Result<(), Error> {
//!
//!     // since this calls windows api, wrap in Arc<Mutex<_>> and share the validator
//!     let app_state = Arc::new(Mutex::new(AppState {
//!         azure_auth: auth::AzureAuth::new("32166c25-5e31-4cfc-a29b-04d0dfdb019a").unwrap(),
//!     }));
//!     println!("Starting web server on: http://localhost:8000");
//!
//!     server::new(move || app(app_state.clone())).bind(port)?.run();
//!
//!     Ok(())
//! }
//! ```
//!
use chrono::{Duration, Local, NaiveDateTime};
use jsonwebtoken as jwt;
use jwt::DecodingKey;
use reqwest::{self, blocking::Response};
use serde::{Deserialize, Serialize};

mod error;
pub use error::AuthErr;

const AZ_OPENID_URL: &str =
    "https://login.microsoftonline.com/common/.well-known/openid-configuration";

/// AzureAuth is the what you'll use to validate your token.
///
/// # Defaults
///
/// - Public key expiration: dafault set to 24h, use `set_expiration` to set a different expiration
///   in hours.
/// - Hashing algorithm: Sha256, you can't change this setting. Submit an issue in the github repo
///   if this is important to you
/// - Retry on no match. If no matching key is found and our keys are older than an hour, we
///   refresh the keys and try once more. Limited to once in an hour. You can disable this by
///   calling `set_no_retry()`.
/// - The timestamps are given a 60s "leeway" to account for time skew between servers
///
/// # Errors:
///
/// - If one of Microsofts enpoints for public keys are down
/// - If the token can't be parsed as a valid Azure token
/// - If the tokens fails it's authenticity test
/// - If the token is invalid
#[derive(Debug, Clone)]
pub struct AzureAuth {
    aud_to_val: String,
    jwks_uri: String,
    public_keys: Option<Vec<Jwk>>,
    last_refresh: Option<NaiveDateTime>,
    exp_hours: i64,
    retry_counter: u32,
    is_retry_enabled: bool,
    is_offline: bool,
}

impl AzureAuth {
    /// Creates a new dafault instance. This method will call the Microsoft apis to fetch the current keys
    /// which can fail. The public keys are fetched since we need them to perform
    /// verification. Please note that fetching the OpenID manifest and public keys are quite slow
    /// since we call an external API in a blocking manner. Try keeping a single instance
    /// alive instead of creating new ones for every validation. If you need to pass around an
    /// instance of the object, creating a pool of instances at startup or wrapping a single
    /// instance in a `Mutex` is better than creating many new instances.
    ///
    /// # Errors
    ///
    /// If there is a connection issue to the Microsoft APIs.
    pub fn new(aud: impl Into<String>) -> Result<Self, AuthErr> {
        Ok(AzureAuth {
            aud_to_val: aud.into(),
            jwks_uri: AzureAuth::get_jwks_uri()?,
            public_keys: None,
            last_refresh: None,
            exp_hours: 24,
            retry_counter: 0,
            is_retry_enabled: true,
            is_offline: false,
        })
    }

    /// Does not call the Microsoft openid configuration endpoint or fetches the JWK set.
    /// Use this if you want to handle updating the public keys yourself
    pub fn new_offline(aud: impl Into<String>, public_keys: Vec<Jwk>) -> Result<Self, AuthErr> {
        Ok(AzureAuth {
            aud_to_val: aud.into(),
            jwks_uri: String::new(),
            public_keys: Some(public_keys),
            last_refresh: Some(Local::now().naive_local()),
            exp_hours: 24,
            retry_counter: 0,
            is_retry_enabled: true,
            is_offline: true,
        })
    }

    /// Dafault validation, see `AzureAuth` documentation for the defaults.
    pub fn validate_token(&mut self, token: &str) -> Result<Token<AzureJwtClaims>, AuthErr> {
        let mut validator = jwt::Validation::new(jwt::Algorithm::RS256);

        // exp, nbf, iat is set to validate as default
        // Since validating time fields is always a bit tricky due to clock skew, you can add some leeway
        validator.leeway = 60;
        validator.set_audience(&[&self.aud_to_val]);
        let decoded: Token<AzureJwtClaims> = self.validate_token_authenticity(token, &validator)?;

        Ok(decoded)
    }

    /// Allows for a custom validator and mapping the token to your own type.
    /// Useful in situations where you get fields you that are not covered by
    /// the default mapping or want to change the validaion requirements (i.e
    /// if you want the leeway set to two minutes instead of one).
    ///
    /// # Note
    /// You'll need to pull in `jsonwebtoken` to use `Validation` from that crate.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use azure_oauth_r1s::*;
    /// use jsonwebtoken::{Validation, Token};
    /// use serde::{Seralize, Deserialize};
    ///
    /// let mut validator = Validation::new();
    /// validator.leeway = 120;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct MyClaims {
    ///     group: String,
    ///     roles: Vec<String>,
    /// }
    ///
    /// let auth = AzureAuth::new(my_client_id_from_azure).unwrap();
    ///
    /// let valid_token: Token<MyClaims>  = auth.validate_custom(some_token, &validator).unwrap();
    /// ```
    ///
    pub fn validate_custom<T>(
        &mut self,
        token: &str,
        validator: &jwt::Validation,
    ) -> Result<Token<T>, AuthErr>
    where
        for<'de> T: Serialize + Deserialize<'de>,
    {
        let decoded: Token<T> = self.validate_token_authenticity(token, validator)?;
        Ok(decoded)
    }

    fn validate_token_authenticity<T>(
        &mut self,
        token: &str,
        validator: &jwt::Validation,
    ) -> Result<Token<T>, AuthErr>
    where
        for<'de> T: Serialize + Deserialize<'de>,
    {
        // if we´re in offline, we never refresh the keys. It's up to the user to do that.
        if !self.is_keys_valid() && !self.is_offline {
            self.refresh_pub_keys()?;
        }
        // does not validate the token!
        let decoded = jwt::decode_header(token)?;

        let key = match &self.public_keys {
            None => return Err(AuthErr::Other("Internal err. No public keys found.".into())),
            Some(keys) => match &decoded.kid {
                None => return Err(AuthErr::Other("No `kid` in token.".into())),
                Some(kid) => keys.iter().find(|k| k.kid == *kid),
            },
        };

        let auth_key = match key {
            None => {
                // the first time this happens let's go and refresh the keys and try once more.
                // It could be that our keys are out of date. Limit to once in an hour.
                if self.should_retry() {
                    self.refresh_pub_keys()?;
                    self.retry_counter += 1;
                    self.validate_token(token)?;
                    unreachable!()
                } else {
                    self.retry_counter = 0;
                    return Err(AuthErr::Other(
                        "Invalid token. Could not verify authenticity.".into(),
                    ));
                }
            }
            Some(key) => {
                self.retry_counter = 0;
                key
            }
        };

        let key = DecodingKey::from_rsa_components(auth_key.modulus(), auth_key.exponent())?;
        let valid: Token<T> = jwt::decode(token, &key, validator)?;

        Ok(valid)
    }

    fn should_retry(&mut self) -> bool {
        if self.is_offline || !self.is_retry_enabled {
            return false;
        }

        match &self.last_refresh {
            Some(lr) => {
                self.retry_counter == 0 && Local::now().naive_local() - *lr > Duration::hours(1)
            }
            None => false,
        }
    }

    /// Sets the expiration of the cached public keys in hours. Pr. 04.2019 Microsoft rotates these
    /// every 24h.
    pub fn set_expiration(&mut self, hours: i64) {
        self.exp_hours = hours;
    }

    pub fn set_no_retry(&mut self) {
        self.is_retry_enabled = false;
    }

    fn is_keys_valid(&self) -> bool {
        match self.last_refresh {
            None => false,
            Some(lr) => (Local::now().naive_local() - lr) <= Duration::hours(self.exp_hours),
        }
    }

    fn refresh_pub_keys(&mut self) -> Result<(), AuthErr> {
        let resp: Response = reqwest::blocking::get(&self.jwks_uri)?;
        let resp: JwkSet = resp.json()?;
        self.last_refresh = Some(Local::now().naive_local());
        self.public_keys = Some(resp.keys);
        Ok(())
    }

    /// Refreshes the jwks_uri by re-fetching it from the the OpenID metadata
    /// document. See: https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
    /// Usually, this is not needed but for some cases you might want to try
    /// to fetch a new uri on receiving an error.
    pub fn refresh_rwks_uri(&mut self) -> Result<(), AuthErr> {
        self.jwks_uri = AzureAuth::get_jwks_uri()?;
        Ok(())
    }

    fn get_jwks_uri() -> Result<String, AuthErr> {
        let resp: Response = reqwest::blocking::get(AZ_OPENID_URL)?;
        let resp: OpenIdResponse = resp.json()?;

        Ok(resp.jwks_uri)
    }

    /// If you use the "offline" variant you'll need this to update the public keys, if you don't
    /// use the offline version you probably don't want to change these unless you're testing.
    pub fn set_public_keys(&mut self, pub_keys: Vec<Jwk>) {
        self.last_refresh = Some(Local::now().naive_local());
        self.public_keys = Some(pub_keys);
    }
}

pub struct AzureJwtHeader {
    /// Indicates that the token is a JWT.
    pub typ: String,
    /// Indicates the algorithm that was used to sign the token. Example: "RS256"
    pub alg: String,
    /// Thumbprint for the public key used to sign this token. Emitted in both
    /// v1.0 and v2.0 id_tokens
    pub kid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AzureJwtClaims {
    /// dentifies the intended recipient of the token. In id_tokens, the audience
    /// is your app's Application ID, assigned to your app in the Azure portal.
    /// Your app should validate this value, and reject the token if the value
    /// does not match.
    pub aud: String,

    /// The application ID of the client using the token. The application can
    /// act as itself or on behalf of a user. The application ID typically
    /// represents an application object, but it can also represent a service
    /// principal object in Azure AD.
    pub azp: Option<String>,

    /// Indicates how the client was authenticated. For a public client, the
    /// value is "0". If client ID and client secret are used, the value is "1".
    /// If a client certificate was used for authentication, the value is "2".
    pub azpacr: Option<String>,

    /// Identifies the security token service (STS) that constructs and returns
    /// the token, and the Azure AD tenant in which the user was authenticated.
    /// If the token was issued by the v2.0 endpoint, the URI will end in /v2.0.
    /// The GUID that indicates that the user is a consumer user from a Microsoft
    /// account is 9188040d-6c67-4c5b-b112-36a304b66dad.
    ///
    /// Your app should use the GUID portion of the claim to restrict the set of
    /// tenants that can sign in to the app, if applicable.
    pub iss: String,

    /// Unix timestamp. "Issued At" indicates when the authentication for this
    /// token occurred.
    pub iat: u64,

    /// Records the identity provider that authenticated the subject of the token.
    /// This value is identical to the value of the Issuer claim unless the user
    /// account not in the same tenant as the issuer - guests, for instance. If
    /// the claim isn't present, it means that the value of iss can be used
    /// instead. For personal accounts being used in an organizational context
    /// (for instance, a personal account invited to an Azure AD tenant), the idp
    /// claim may be 'live.com' or an STS URI containing the Microsoft account
    /// tenant 9188040d-6c67-4c5b-b112-36a304b66dad
    pub idp: Option<String>,

    /// Unix timestamp. The "nbf" (not before) claim identifies the time before
    /// which the JWT MUST NOT be accepted for processing.
    pub nbf: u64,

    /// Unix timestamp. he "exp" (expiration time) claim identifies the
    /// expiration time on or after which the JWT MUST NOT be accepted for
    /// processing. It's important to note that a resource may reject the token
    /// before this time as well - if, for example, a change in authentication
    /// is required or a token revocation has been detected.
    pub exp: u64,

    /// The code hash is included in ID tokens only when the ID token is issued
    /// with an OAuth 2.0 authorization code. It can be used to validate the
    /// authenticity of an authorization code. For details about performing this
    /// validation, see the OpenID Connect specification.
    pub c_hash: Option<String>,

    /// The access token hash is included in ID tokens only when the ID token is
    /// issued with an OAuth 2.0 access token. It can be used to validate the
    /// authenticity of an access token. For details about performing this
    /// validation, see the OpenID Connect specification.
    pub at_hash: Option<String>,

    /// The email claim is present by default for guest accounts that have an
    /// email address. Your app can request the email claim for managed users
    /// (those from the same tenant as the resource) using the email optional
    /// claim. On the v2.0 endpoint, your app can also request the email OpenID
    /// Connect scope - you don't need to request both the optional claim and
    /// the scope to get the claim. The email claim only supports addressable
    /// mail from the user's profile information.
    pub preferred_username: Option<String>,

    /// The name claim provides a human-readable value that identifies the
    /// subject of the token. The value isn't guaranteed to be unique, it is
    /// mutable, and it's designed to be used only for display purposes. The
    /// profile scope is required to receive this claim.
    pub name: Option<String>,

    /// The nonce matches the parameter included in the original /authorize
    /// request to the IDP. If it does not match, your application should reject
    /// the token.
    pub nonce: Option<String>,

    /// Guid. The immutable identifier for an object in the Microsoft identity system,
    /// in this case, a user account. This ID uniquely identifies the user
    /// across applications - two different applications signing in the same
    /// user will receive the same value in the oid claim. The Microsoft Graph
    /// will return this ID as the id property for a given user account. Because
    /// the oid allows multiple apps to correlate users, the profile scope is
    /// required to receive this claim. Note that if a single user exists in
    /// multiple tenants, the user will contain a different object ID in each
    /// tenant - they're considered different accounts, even though the user
    /// logs into each account with the same credentials.
    pub oid: String,

    /// The set of roles that were assigned to the user who is logging in.
    pub roles: Option<Vec<String>>,

    /// The set of scopes exposed by your application for which the client
    /// application has requested (and received) consent. Your app should verify
    /// that these scopes are valid ones exposed by your app, and make authorization
    /// decisions based on the value of these scopes. Only included for user tokens.
    pub scp: Option<String>,

    /// The principal about which the token asserts information, such as the
    /// user of an app. This value is immutable and cannot be reassigned or
    /// reused. The subject is a pairwise identifier - it is unique to a
    /// particular application ID. If a single user signs into two different
    /// apps using two different client IDs, those apps will receive two
    /// different values for the subject claim. This may or may not be wanted
    /// depending on your architecture and privacy requirements.
    pub sub: String,

    /// A GUID that represents the Azure AD tenant that the user is from.
    /// For work and school accounts, the GUID is the immutable tenant ID of
    /// the organization that the user belongs to. For personal accounts,
    /// the value is 9188040d-6c67-4c5b-b112-36a304b66dad. The profile scope is
    /// required to receive this claim.
    pub tid: String,

    /// Provides a human readable value that identifies the subject of the
    /// token. This value isn't guaranteed to be unique within a tenant and
    /// should be used only for display purposes. Only issued in v1.0 id_tokens.
    pub unique_name: Option<String>,

    /// Indicates the version of the id_token. Either 1.0 or 2.0.
    pub ver: String,
}

#[derive(Debug, Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
}

impl Jwk {
    fn modulus(&self) -> &str {
        &self.n
    }

    fn exponent(&self) -> &str {
        &self.e
    }
}

#[derive(Deserialize)]
struct OpenIdResponse {
    jwks_uri: String,
}

type Token<T> = jwt::TokenData<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_rwks_uri() {
        let _az_auth = AzureAuth::new("app_secret").unwrap();
    }

    #[test]
    fn azure_ad_get_public_keys() {
        let mut az_auth = AzureAuth::new("app_secret").unwrap();
        az_auth.refresh_pub_keys().unwrap();
    }

    #[test]
    fn azure_ad_get_refresh_rwks_uri() {
        let mut az_auth = AzureAuth::new("app_secret").unwrap();
        az_auth.refresh_rwks_uri().unwrap();
    }

    #[test]
    fn is_not_valid_more_than_24h() {
        let mut az_auth = AzureAuth::new("app_secret").unwrap();
        az_auth.last_refresh = Some(Local::now().naive_local() - Duration::hours(25));

        assert!(!az_auth.is_keys_valid());
    }
}
