use serde::Serialize;
use sqlx::FromRow;

use super::Database;

#[derive(FromRow)]
pub struct ImageData {
    pub id: i64,
    pub mime: String,
    pub content: Vec<u8>,
}

#[derive(FromRow, Serialize)]
pub struct BookImage {
    pub id: String,
    #[serde(skip)]
    pub book_id: String,
    pub file_name: String,
    pub page_number: i32,
    #[serde(skip)]
    pub image_id: i64,
}

pub async fn create_image_data(db: &Database, image_data: &ImageData) -> Result<i64, sqlx::Error> {
    let id = sqlx::query("INSERT INTO ImagesData (mime, content) VALUES (?, ?)")
        .bind(&image_data.mime)
        .bind(&image_data.content)
        .execute(db.get_pool())
        .await?
        .last_insert_id();
    Ok(id as i64)
}

pub async fn create_book_image(db: &Database, image: &BookImage) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO BookImages (id, book_id, page_number, image_id, file_name) VALUES (?, ?, ?, ?, ?)")
        .bind(&image.id)
        .bind(&image.book_id)
        .bind(&image.page_number)
        .bind(&image.image_id)
        .bind(&image.file_name)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_book_image(db: &Database, id: &String, img_id: &String) -> Result<Option<BookImage>, sqlx::Error> {
    let book = sqlx::query_as::<_, BookImage>("SELECT * FROM BookImages WHERE book_id = ? AND id = ?")
        .bind(id)
        .bind(img_id)
        .fetch_optional(db.get_pool())
        .await?;
    Ok(book)
}

pub async fn get_image_data(db: &Database, id: &i64) -> Result<Option<ImageData>, sqlx::Error> {
    let image = sqlx::query_as::<_, ImageData>("SELECT * FROM ImagesData WHERE id = ?")
        .bind(id)
        .fetch_optional(db.get_pool())
        .await?;
    Ok(image)
}
