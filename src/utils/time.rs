use serde::{self, Deserialize};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[allow(unused)]
pub fn serialize_offset_date_time<S>(
    date_time: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date_time {
        Some(dt) => serializer.serialize_str(
            &dt.format(&Iso8601::DEFAULT)
                .map_err(serde::ser::Error::custom)?,
        ),
        None => serializer.serialize_none(),
    }
}

#[allow(unused)]
pub fn deserialize_offset_date_time<'de, D>(
    deserializer: D,
) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(
            OffsetDateTime::parse(&s, &Iso8601::DEFAULT).map_err(serde::de::Error::custom)?,
        )),
        None => Ok(None),
    }
}
