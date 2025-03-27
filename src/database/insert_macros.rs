#[macro_export]
macro_rules! insert_resource {
    ($resource:ty, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use pluralizer::pluralize;
        use uuid::Uuid;

        async {
            let id = Uuid::new_v4().to_string();

            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let mut params: Vec<(String, DatabaseValue)> = Vec::new();
            for (field, value) in $params.into_iter() {
                params.push((field.to_string(), DatabaseValue::String(value.to_string())));
            }
            if <$resource as DatabaseResource>::has_id() {
                params.push(("id".to_string(), DatabaseValue::String(id.clone())));
            }

            let fields: Vec<String> = params.iter().map(|(field, _)| field.clone()).collect();
            let values: Vec<&DatabaseValue> = params.iter().map(|(_, value)| value).collect();

            let mut query = format!("INSERT INTO {} (", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(field);
                if i < fields.len() - 1 {
                    query.push_str(", ");
                }
            }
            query.push_str(") VALUES (");
            for (i, _) in values.iter().enumerate() {
                query.push_str(&format!("${}", i + 1));
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
