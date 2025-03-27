use sqlx::postgres::PgArgumentBuffer;
use sqlx::{encode::IsNull, error::BoxDynError, Encode, Postgres, Type};
use std::fmt::{self, Display};
use std::iter::FromIterator;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub enum DatabaseValue {
    #[allow(dead_code)]
    None,
    #[allow(dead_code)]
    Str(&'static str),
    #[allow(dead_code)]
    String(String),
    #[allow(dead_code)]
    Int(i32),
    #[allow(dead_code)]
    Int64(i64),
    #[allow(dead_code)]
    Float(f64),
    #[allow(dead_code)]
    Boolean(bool),
    #[allow(dead_code)]
    DateTime(OffsetDateTime),
}

impl Display for DatabaseValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'q> Encode<'q, Postgres> for DatabaseValue {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        match self {
            DatabaseValue::None => Ok(IsNull::Yes),
            DatabaseValue::Str(s) => Encode::<Postgres>::encode_by_ref(s, buf),
            DatabaseValue::String(s) => Encode::<Postgres>::encode_by_ref(s, buf),
            DatabaseValue::Int(i) => Encode::<Postgres>::encode_by_ref(i, buf),
            DatabaseValue::Int64(i) => Encode::<Postgres>::encode_by_ref(i, buf),
            DatabaseValue::Float(f) => Encode::<Postgres>::encode_by_ref(f, buf),
            DatabaseValue::Boolean(b) => Encode::<Postgres>::encode_by_ref(b, buf),
            DatabaseValue::DateTime(dt) => Encode::<Postgres>::encode_by_ref(dt, buf),
        }
    }
}

impl Type<Postgres> for DatabaseValue {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        // Most general type that can handle all our variants
        sqlx::postgres::PgTypeInfo::with_name("text")
    }
}

impl<'a> FromIterator<&'a str> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        DatabaseValue::String(iter.into_iter().collect::<String>())
    }
}

impl FromIterator<String> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        DatabaseValue::String(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a String> for DatabaseValue {
    fn from_iter<I: IntoIterator<Item = &'a String>>(iter: I) -> Self {
        DatabaseValue::String(iter.into_iter().cloned().collect())
    }
}
