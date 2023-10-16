use sqlx::{MySql, Pool};

use crate::db;

use anyhow::Result;

pub async fn list_all_services(pool: &Pool<MySql>) -> Result<Vec<String>> {
    let mut tx = pool.begin().await?;

    let result = db::get_all_services(&mut tx).await?;

    let mut services = Vec::new();

    for service in result {
        services.push(service.name.unwrap());
    }

    Ok(services)
}
