use serde::{Deserialize, Deserializer, de::Error};

pub(crate) fn empty_as_none<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Helper<T> {
        Value(T),
        String(String),
    }

    match Helper::<T>::deserialize(deserializer)? {
        Helper::Value(value) => Ok(Some(value)),
        Helper::String(s) if s.is_empty() => Ok(None),
        Helper::String(_) => Err(D::Error::custom("expected empty string")),
    }
}