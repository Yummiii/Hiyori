use crate::database::Database;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Clone, Serialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing)]
    pub cover: i64,
    pub collection: String,
}

pub async fn create_book(db: &Database, book: &Book) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO Books (id, title, cover, collection) VALUES (?, ?, ?, ?)")
        .bind(&book.id)
        .bind(&book.title)
        .bind(&book.cover)
        .bind(&book.collection)
        .execute(db.get_pool())
        .await?;
    Ok(())
}

pub async fn get_book(db: &Database, id: &str) -> Option<Book> {
    let book = sqlx::query_as::<_, Book>("SELECT * FROM Books WHERE id = ?")
        .bind(id)
        .fetch_optional(db.get_pool())
        .await
        .unwrap();
    book
}

pub async fn get_books_by_collection(db: &Database, collection: &String) -> Vec<Book> {
    let books =
        sqlx::query_as::<_, Book>("SELECT * FROM Books WHERE collection = ? ORDER BY title ASC")
            .bind(collection)
            .fetch_all(db.get_pool())
            .await
            .unwrap();
    books
}

pub async fn delete_book(db: &Database, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM Books WHERE id = ?")
        .bind(id)
        .execute(db.get_pool())
        .await?;
    Ok(())
}
