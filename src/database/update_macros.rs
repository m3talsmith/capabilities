#[macro_export]
macro_rules! update_resource {
    ($resource:ty, $id:expr, $params:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use pluralizer::pluralize;
        use time::{format_description::well_known::Rfc3339, OffsetDateTime};

        async {
            let updated_at = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();

            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let params = $params.clone();

            let fields = params
                .iter()
                .map(|(field, _)| field.to_string())
                .collect::<Vec<String>>();
            let values = params
                .iter()
                .map(|(_, value)| DatabaseValue::String(value.to_string()))
                .collect::<Vec<DatabaseValue>>();

            let mut query = format!("UPDATE {} SET ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                if field.to_lowercase().contains("_at") {
                    query.push_str(&format!("{} = CAST(${} AS TIMESTAMP)", field, i + 1));
                } else {
                    query.push_str(&format!("{} = ${}", field, i + 1));
                }
                if i < fields.len() - 1 {
                    query.push_str(", ");
                }
            }
            if <$resource as DatabaseResource>::is_updatable() {
                query.push_str(&format!(
                    ", updated_at = CAST(${} AS TIMESTAMP)",
                    fields.len() + 1
                ));
            }
            query.push_str(&format!(" WHERE id = ${}", fields.len() + 1));
            query.push_str(&format!(" RETURNING *"));

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            if <$resource as DatabaseResource>::is_updatable() {
                query = query.bind(updated_at);
            }

            match query.execute(&pool).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            };

            let id = DatabaseValue::String($id.to_string());
            let params = vec![("id", &id)];
            match find_one_resource_where_fields!($resource, params).await {
                Ok(resource) => Ok(resource),
                Err(e) => Err(e),
            }
        }
    }};
}
