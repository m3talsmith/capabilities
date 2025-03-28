#[macro_export]
macro_rules! delete_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::connection::get_connection;
        use crate::database::traits::DatabaseResource;
        use crate::database::values::DatabaseValue;
        use anyhow::anyhow;
        use pluralizer::pluralize;
        use time::OffsetDateTime;
        async {
            let archived_at = OffsetDateTime::now_utc();

            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let params = $params.clone();

            let fields = params
                .iter()
                .map(|field| field.0.clone().to_string())
                .collect::<Vec<String>>();
            let values = params
                .iter()
                .map(|field| DatabaseValue::String(field.1.clone()))
                .collect::<Vec<DatabaseValue>>();

            let mut query: String;
            if <$resource as DatabaseResource>::is_archivable() {
                query = format!(
                    "UPDATE {} SET archived_at = CAST(${} AS TIMESTAMP) WHERE ",
                    resource_name,
                    fields.len() + 1
                );
            } else {
                query = format!("DELETE FROM {} WHERE ", resource_name);
            }

            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            if <$resource as DatabaseResource>::is_archivable() {
                query = query.bind(archived_at);
            }

            match query.execute(&pool).await {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow!(e)),
            }
        }
    }};
}
