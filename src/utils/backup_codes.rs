use rand::{rng, Rng};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

use crate::find_all_resources_where_fields;
use crate::models::backup_code::BackupCode;

fn generate_code() -> String {
    let mut rng = rng();
    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let code = format!("{:06}", rng.random_range(0..1000000));
    let hash = Sha256::digest(format!("{:?}{}", timestamp, code).as_bytes()).to_vec();
    hash[..7]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

pub async fn generate_backup_code() -> String {
    let backup_code = generate_code();
    match find_all_resources_where_fields!(
        BackupCode,
        vec![("code", DatabaseValue::String(backup_code.clone()))]
    )
    .await
    {
        Ok(backup_codes) => {
            if backup_codes.is_empty() {
                backup_code
            } else {
                Box::pin(generate_backup_code()).await
            }
        }
        Err(err) => {
            println!("Error generating backup code: {:?}", err);
            Box::pin(generate_backup_code()).await
        }
    }
}

pub async fn generate_backup_codes() -> Vec<String> {
    let mut codes = Vec::new();
    for _ in 0..10 {
        let code = generate_backup_code().await;
        codes.push(code);
    }
    codes
}
