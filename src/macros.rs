macro_rules! impl_deserialize {
    ($for_ty: ident) => {
        impl<'de> serde::Deserialize<'de> for $for_ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de;
                let s = <String as serde::Deserialize>::deserialize(deserializer)?;
                <$for_ty>::new(s.as_str()).map_err(|e| de::Error::custom(e.to_string()))
            }
        }
    };
}
pub(crate) use impl_deserialize;
