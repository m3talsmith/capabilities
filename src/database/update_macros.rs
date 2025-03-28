#[macro_export]
macro_rules! update_resource {
    ($resource:ty, $id:expr, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use pluralizer::pluralize;
        use time::{format_description::well_known::Iso8601, Duration, OffsetDateTime};

        async {
            let updated_at = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap();
            let expires_at = (OffsetDateTime::now_utc() + Duration::days(30))
                .format(&Iso8601::DEFAULT)
                .unwrap();

            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let mut params: Vec<(String, DatabaseValue)> = Vec::new();

            let input_params: Vec<(String, DatabaseValue)> = $params;
            if !input_params.is_empty() {
                for (field, value) in input_params {
                    params.push((field, value.clone()));
                }
            }
            if <$resource as DatabaseResource>::is_updatable() {
                params.push(("updated_at".to_string(), DatabaseValue::String(updated_at)));
            }
            if <$resource as DatabaseResource>::is_expirable() {
                params.push(("expires_at".to_string(), DatabaseValue::String(expires_at)));
            }

            let fields = params
                .iter()
                .map(|(field, _)| field.to_string())
                .collect::<Vec<String>>();
            let values: Vec<&DatabaseValue> = params.iter().map(|(_, value)| value).collect();

            let mut query = format!("UPDATE {} SET ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                match field.contains("_at") {
                    true => query.push_str(&format!("{} = CAST(${} AS TIMESTAMP)", field, i + 1)),
                    _ => query.push_str(&format!("{} = ${}", field, i + 1)),
                }
                if i < fields.len() - 1 {
                    query.push_str(", ");
                }
            }
            query.push_str(&format!(" WHERE id = ${}", fields.len() + 1));
            query.push_str(&format!(" RETURNING *"));

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            query = query.bind(&$id);

            match query.execute(&pool).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            };

            let params = vec![("id", &$id)];
            match find_one_resource_where_fields!($resource, params).await {
                Ok(resource) => Ok(resource),
                Err(e) => Err(e),
            }
        }
    }};
}
