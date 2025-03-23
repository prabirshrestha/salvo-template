use std::str::FromStr;

use surrealdb::sql::Datetime;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub fn to_surreal_datetime(datetime: &OffsetDateTime) -> Datetime {
    // NOTE: remove this util function once time crate is official supported.
    // https://github.com/surrealdb/surrealdb/issues/2563
    Datetime::from_str(&datetime.format(&Rfc3339).unwrap()).unwrap()
}
