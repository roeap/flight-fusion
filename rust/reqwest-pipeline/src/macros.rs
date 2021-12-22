#[macro_export]
macro_rules! setters {
    (@single $name:ident : $typ:ty => $transform:expr) => {
        #[allow(clippy::redundant_field_names)]
        #[allow(clippy::needless_update)]
        // TODO: Declare using idiomatic with_$name when https://github.com/Azure/azure-sdk-for-rust/issues/292 is resolved.
        pub fn $name<T: ::std::convert::Into<$typ>>(self, $name: T) -> Self {
            let $name: $typ = $name.into();
            Self  {
                $name: $transform,
                ..self
            }
        }
    };
    // Terminal condition
    (@recurse) => {};
    // Recurse without transform
    (@recurse $name:ident : $typ:ty, $($tokens:tt)*) => {
        setters! { @recurse $name: $typ => $name, $($tokens)* }
    };
    // Recurse with transform
    (@recurse $name:ident : $typ:ty => $transform:expr, $($tokens:tt)*) => {
        setters! { @single $name : $typ => $transform }
        setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        setters! { @recurse $($tokens)* }
    }
}

#[macro_export]
macro_rules! query_prop {
    (@single $name:ident : $typ:ty => $type_name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $type_name($typ);

        impl $type_name {
            pub fn new(value: $typ) -> Self {
                Self(value)
            }
        }

        impl AppendToUrlQuery for $type_name {
            fn append_to_url_query(&self, url: &mut url::Url) {
                url.query_pairs_mut()
                    .append_pair(stringify! {$name}, &format!("{}", self.0));
            }
        }

        impl From<$typ> for $type_name {
            fn from(value: $typ) -> Self {
                Self::new(value)
            }
        }
    };
    // Terminal condition
    (@recurse) => {};
    // Recurse with transform
    (@recurse $name:ident : $typ:ty => $type_name:ident, $($tokens:tt)*) => {
        query_prop! { @single $name : $typ => $type_name }
        query_prop! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        query_prop! { @recurse $($tokens)* }
    }
}
