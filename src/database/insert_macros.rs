#[macro_export]
macro_rules! insert_resource {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;
        use time::{format_description::well_known::Iso8601, Duration, OffsetDateTime};
        use uuid::Uuid;

        async {
            let id = Uuid::new_v4().to_string();
            let created_at = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap();
            let updated_at = created_at.clone();
            let expires_at = (OffsetDateTime::now_utc() + Duration::days(30))
                .format(&Iso8601::DEFAULT)
                .unwrap();

            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let mut params: Vec<(String, DatabaseValue)> = Vec::new();
            for (field, value) in $params.into_iter() {
                params.push((field.to_string(), value.clone()))
            }

            if <$resource as DatabaseResource>::has_id() {
                params.push(("id".to_string(), DatabaseValue::String(id.clone())));
            }
            if <$resource as DatabaseResource>::is_creatable() {
                params.push(("created_at".to_string(), DatabaseValue::String(created_at)));
            }
            if <$resource as DatabaseResource>::is_updatable() {
                params.push(("updated_at".to_string(), DatabaseValue::String(updated_at)));
            }
            if <$resource as DatabaseResource>::is_expirable() {
                params.push(("expires_at".to_string(), DatabaseValue::String(expires_at)));
            }

            let fields: Vec<String> = params.iter().map(|(field, _)| field.clone()).collect();
            let values: Vec<DatabaseValue> =
                params.iter().map(|(_, value)| (*value).clone()).collect();

            let mut query = format!("INSERT INTO {} (", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(field);
                if i < fields.len() - 1 {
                    query.push_str(", ");
                }
            }

            query.push_str(") VALUES (");
            for (i, _) in values.iter().enumerate() {
                match fields[i].contains("_at") {
                    true => query.push_str(&format!("CAST(${} AS TIMESTAMP)", i + 1)),
                    _ => query.push_str(&format!("${}", i + 1)),
                }
                if i < values.len() - 1 {
                    query.push_str(", ");
                }
            }
            query.push_str(") RETURNING *");

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }

            match query.fetch_one(&pool).await {
                Ok(row) => Ok(<$resource as DatabaseResource>::from_row(&row)?),
                Err(e) => Err(e),
            }
        }
    }};
}
