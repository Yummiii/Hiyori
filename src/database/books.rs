use super::{images::BookImage, Database};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub collection_id: String,
}

pub async fn create_book(db: &Database, book: &Book) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO Books (id, name, collection_id) VALUES (?, ?, ?)")
        .bind(&book.id)
        .bind(&book.name)
        .bind(&book.collection_id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_book(db: &Database, id: &String) -> Result<Option<Book>, sqlx::Error> {
    let book = sqlx::query_as::<_, Book>("SELECT * FROM Books WHERE id = ?")
        .bind(id)
        .fetch_optional(db.get_pool())
        .await?;
    Ok(book)
}

pub async fn get_book_pages(db: &Database, id: &String) -> Result<Vec<BookImage>, sqlx::Error> {
    let pages = sqlx::query_as::<_, BookImage>("SELECT * FROM BookImages WHERE book_id = ? ORDER BY page_number ASC")
        .bind(id)
        .fetch_all(db.get_pool())
        .await?;
    Ok(pages)
}

pub async fn get_books_by_collection(db: &Database, collection_id: &String) -> Result<Vec<Book>, sqlx::Error> {
    let books = sqlx::query_as::<_, Book>("SELECT * FROM Books WHERE collection_id = ? ORDER BY name ASC")
        .bind(collection_id)
        .fetch_all(db.get_pool())
        .await?;
    Ok(books)
}

pub async fn get_book_images(db: &Database, book_id: &String) -> Result<Vec<BookImage>, sqlx::Error> {
    let images = sqlx::query_as::<_, BookImage>("SELECT * FROM BookImages WHERE book_id = ? ORDER BY page_number ASC")
        .bind(book_id)
        .fetch_all(db.get_pool())
        .await?;
    Ok(images)
}