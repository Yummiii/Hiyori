use crate::database::Database;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct Image {
    pub id: String,
    #[serde(skip_serializing)]
    pub book: String,
    pub page: i32,
    #[serde(skip_serializing)]
    pub data: i64,
}

#[derive(FromRow)]
pub struct ImageData {
    pub id: i64,
    pub mime: String,
    pub content: Vec<u8>,
}

pub async fn create_image_data(db: &Database, data: &ImageData) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO ImagesData (mime, content) VALUES (?, ?)")
        .bind(&data.mime)
        .bind(&data.content)
        .execute(db.get_pool())
        .await?;
    Ok(result.last_insert_id() as i64)
}

pub async fn create_image(db: &Database, image: &Image) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO Images (id, book, page, data) VALUES (?, ?, ?, ?)")
        .bind(&image.id)
        .bind(&image.book)
        .bind(&image.page)
        .bind(&image.data)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_image_data(db: &Database, id: i64) -> Option<ImageData> {
    let data = sqlx::query_as::<_, ImageData>("SELECT * FROM ImagesData WHERE id = ?")
        .bind(id)
        .fetch_optional(db.get_pool())
        .await
        .unwrap();
    data
}

pub async fn get_image_data_by_image_id(db: &Database, id: &String) -> Option<ImageData> {
    let image = sqlx::query_as::<_, ImageData>("SELECT ImagesData.id, mime, content from Images join ImagesData on Images.data = ImagesData.id where Images.id = ?")
        .bind(id)
        .fetch_optional(db.get_pool())
        .await
        .unwrap();
    image
}

pub async fn get_images_by_book(db: &Database, book: &String) -> Vec<Image> {
    let images =
        sqlx::query_as::<_, Image>("SELECT * FROM Images WHERE book = ? ORDER BY page ASC")
            .bind(book)
            .fetch_all(db.get_pool())
            .await
            .unwrap();
    images
}

pub async fn delete_image_data(db: &Database, id: &i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM ImagesData WHERE id = ?")
        .bind(id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}
