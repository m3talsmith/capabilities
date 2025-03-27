#[macro_export]
macro_rules! find_all_resources_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!("SELECT * FROM {} WHERE ", resource_name);
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
            match query.fetch_all(&pool).await {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(&row))
                    .collect::<Result<Vec<$resource>, _>>(),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_all_unarchived_resources_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!("SELECT * FROM {} WHERE archived_at IS NULL ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("AND {} = ${}", field, i + 1));
            }

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            match query.fetch_all(&pool).await {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(&row))
                    .collect::<Result<Vec<$resource>, _>>(),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_all_archived_resources_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NOT NULL ",
                resource_name
            );
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("AND {} = ${}", field, i + 1));
            }

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            match query.fetch_all(&pool).await {
                Ok(rows) => rows
                    .into_iter()
                    .map(|row| <$resource as DatabaseResource>::from_row(&row))
                    .collect::<Result<Vec<$resource>, _>>(),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_one_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!("SELECT * FROM {} WHERE ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(" LIMIT 1");

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            match query.fetch_one(&pool).await {
                Ok(row) => <$resource as DatabaseResource>::from_row(&row),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_one_unarchived_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!("SELECT * FROM {} WHERE archived_at IS NULL ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("AND {} = ${}", field, i + 1));
            }
            query.push_str(" LIMIT 1");

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            match query.fetch_one(&pool).await {
                Ok(row) => <$resource as DatabaseResource>::from_row(&row),
                Err(e) => Err(e),
            }
        }
    }};
}

#[macro_export]
macro_rules! find_one_archived_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NOT NULL ",
                resource_name
            );
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("AND {} = ${}", field, i + 1));
            }
            query.push_str(" LIMIT 1");
            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            match query.fetch_one(&pool).await {
                Ok(row) => <$resource as DatabaseResource>::from_row(&row),
                Err(e) => Err(e),
            }
        }
    }};
}
