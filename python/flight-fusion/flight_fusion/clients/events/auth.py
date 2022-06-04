from typing import NamedTuple, Protocol

from grpclib.events import SendRequest

AUTH_HEADER_KEY = "authorization"


class AccessToken(NamedTuple):
    token: str
    expires_on: int


class TokenCredential(Protocol):
    """Protocol for classes able to provide OAuth tokens.

    :param str scopes: Lets you specify the type of access needed.
    """

    def get_token(self, *scopes, **kwargs) -> AccessToken:
        ...


class SendrequestAuth:
    def __init__(self, credential: TokenCredential, scopes: list[str], header_key: str = AUTH_HEADER_KEY):
        self._credential = credential
        self._scopes = scopes
        self._key = header_key

    async def __call__(self, event: SendRequest) -> None:
        token = self._credential.get_token(*self._scopes)
        event.metadata[self._key] = token.token
