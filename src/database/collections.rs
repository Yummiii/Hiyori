use super::Database;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Collection {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    #[serde(skip)]
    pub thumbnail_id: Option<i64>,
}

pub async fn create_collection(db: &Database, collection: &Collection) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO Collections (id, name, thumbnail_id) VALUES (?, ?, ?)")
        .bind(&collection.id)
        .bind(&collection.name)
        .bind(&collection.thumbnail_id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_collection(db: &Database, id: &String) -> Result<Option<Collection>, sqlx::Error> {
    let res = sqlx::query_as::<_, Collection>("SELECT * FROM Collections WHERE id = ?")
        .bind(id)
        .fetch_optional(db.get_pool())
        .await?;
    Ok(res)
}

pub async fn get_collections(db: &Database) -> Result<Vec<Collection>, sqlx::Error> {
    let res = sqlx::query_as::<_, Collection>("SELECT * FROM Collections")
        .fetch_all(db.get_pool())
        .await?;
    Ok(res)
}

pub async fn set_collection_thumbnail(db: &Database, collection_id: &String, thumbnail_id: &i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE Collections SET thumbnail_id = ? WHERE id = ?")
        .bind(thumbnail_id)
        .bind(collection_id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn delete_collection(db: &Database, collection_id: &String) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM Collections WHERE id = ?")
        .bind(collection_id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}