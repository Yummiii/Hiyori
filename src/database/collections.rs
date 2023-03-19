use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::Database;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub thumb: Option<i64>,
}

pub async fn create_collection(db: &Database, collection: &Collection) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO Collections (id, name, thumb) VALUES (?, ?, ?)")
        .bind(&collection.id)
        .bind(&collection.name)
        .bind(&collection.thumb)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_collections(db: &Database) -> Result<Vec<Collection>, sqlx::Error> {
    let collections = sqlx::query_as("SELECT * FROM Collections")
        .fetch_all(db.get_pool())
        .await?;
    Ok(collections)
}

pub async fn get_collection(db: &Database, id: &String) -> Option<Collection> {
    let collection = sqlx::query_as("SELECT * FROM Collections WHERE id = ?")
        .bind(id)
        .fetch_one(db.get_pool())
        .await;

    match collection {
        Ok(collection) => Some(collection),
        Err(_) => None,
    }
}

pub async fn delete_collection(db: &Database, id: &String) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM Collections WHERE id = ?")
        .bind(id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}
