#[macro_export]
macro_rules! find_all_resources_where_fields {
    ($resource:ty, $params:expr) => {{
        find_all_resources_where_fields!($resource, $params, "created_at ASC")
    }};
    ($resource:ty, $params:expr, $order_by:expr) => {{
        use crate::database::{
            connection::get_connection, traits::DatabaseResource, values::DatabaseValue,
        };
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params
                .iter()
                .map(|field| field.1.clone())
                .collect::<Vec<DatabaseValue>>();
            let mut query = format!("SELECT * FROM {} WHERE ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(&format!(" ORDER BY {}", $order_by));

            let mut query = sqlx::query(&query);
            for value in values.iter() {
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
        find_all_unarchived_resources_where_fields!($resource, $params, "created_at ASC")
    }};
    ($resource:ty, $params:expr, $order_by:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NULL AND ",
                resource_name
            );
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(&format!(" ORDER BY {}", $order_by));
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
        find_all_archived_resources_where_fields!($resource, $params, "created_at ASC")
    }};
    ($resource:ty, $params:expr, $order_by:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NOT NULL AND ",
                resource_name
            );
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(&format!(" ORDER BY {}", $order_by));

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
        find_one_resource_where_fields!($resource, $params, "created_at ASC")
    }};
    ($resource:ty, $params:expr, $order_by:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
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
            query.push_str(&format!(" ORDER BY {} LIMIT 1", $order_by));

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
        find_one_unarchived_resource_where_fields!($resource, $params, "created_at ASC")
    }};
    ($resource:ty, $params:expr, $order_by:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();
            let values = $params.iter().map(|field| &field.1).collect::<Vec<_>>();
            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NULL AND ",
                resource_name
            );
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(&format!(" ORDER BY {} LIMIT 1", $order_by));

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
        find_one_archived_resource_where_fields!($resource, $params, "created_at ASC")
    }};
    ($resource:ty, $params:expr, $order_by:expr) => {{
        use crate::database::{connection::get_connection, traits::DatabaseResource};
        use crate::utils::strings::camel_to_snake_case;
        use pluralizer::pluralize;

        async {
            let resource_name = pluralize(
                camel_to_snake_case(stringify!($resource).to_string()).as_str(),
                2,
                false,
            );
            let pool = get_connection().await;

            let mut query = format!(
                "SELECT * FROM {} WHERE archived_at IS NOT NULL AND ",
                resource_name
            );

            let fields = $params
                .iter()
                .map(|field| field.0.to_string())
                .collect::<Vec<String>>();

            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }
            query.push_str(&format!(" ORDER BY {} LIMIT 1", $order_by));

            let mut query = sqlx::query(&query);
            for (_, value) in $params.iter().enumerate() {
                query = query.bind(value.1);
            }

            match query.fetch_one(&pool).await {
                Ok(row) => Ok(<$resource as DatabaseResource>::from_row(&row)?),
                Err(e) => Err(e),
            }
        }
    }};
}
