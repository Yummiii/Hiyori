use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{
    get, post,
    web::{Data, Path},
    HttpResponse,
};
use cuid2::cuid;
use epub::doc::EpubDoc;
use itertools::Itertools;

use crate::database::{
    books::{self, Book},
    collections,
    images::{self, BookImage, ImageData},
    Database,
};

#[derive(MultipartForm)]
pub struct FromEpubRequest {
    pub epub: TempFile,
    pub collection: Text<String>,
}

#[post("/from_epub")]
pub async fn from_epub(db: Data<Database>, epub: MultipartForm<FromEpubRequest>) -> HttpResponse {
    if let Some(content_type) = &epub.epub.content_type {
        if content_type.to_string() != "application/epub+zip" {
            return HttpResponse::BadRequest().body("File is not an epub");
        }
    }

    let collection = epub.collection.0.clone();
    let collection = if collections::get_collection(&db, &collection)
        .await
        .unwrap()
        .is_none()
    {
        return HttpResponse::BadRequest().body("Collection does not exist");
    } else {
        collection
    };

    let mut doc = EpubDoc::from_reader(epub.epub.file.as_file()).unwrap();

    let book = Book {
        id: cuid(),
        name: doc.mdata("title").unwrap(),
        collection_id: collection,
    };
    books::create_book(&db, &book).await.unwrap();

    let mut i = 0;
    let resources = doc.resources.clone();
    for res in resources.iter().sorted() {
        if res.1 .1 == "image/jpeg" || res.1 .1 == "image/png" {
            let img = doc.get_resource_by_path(&res.1 .0).unwrap();
            let img = images::create_image_data(
                &db,
                &ImageData {
                    id: 0,
                    mime: res.1 .1.clone(),
                    content: img,
                },
            )
            .await
            .unwrap();

            images::create_book_image(
                &db,
                &BookImage {
                    id: cuid(),
                    book_id: book.id.clone(),
                    page_number: i,
                    image_id: img,
                    file_name: res.1 .0.file_name().unwrap().to_str().unwrap().to_string(),
                },
            )
            .await
            .unwrap();

            i += 1;
        }
    }

    HttpResponse::Created().json(book)
}

#[get("/{id}")]
pub async fn get_book(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let book = books::get_book(&db, &id).await.unwrap();
    let book = match book {
        Some(book) => book,
        None => return HttpResponse::NotFound().body("Book not found"),
    };

    let pages = books::get_book_pages(&db, &id).await.unwrap();

    HttpResponse::Ok().json(serde_json::json!(
        {
            "book": book,
            "pages": pages
        }
    ))
}

#[get("/{id}/images/{img_id}")]
pub async fn get_book_image(db: Data<Database>, ids: Path<(String, String)>) -> HttpResponse {
    let book = books::get_book(&db, &ids.0).await.unwrap();
    let book = match book {
        Some(book) => book,
        None => return HttpResponse::NotFound().body("Book not found"),
    };

    let img = images::get_book_image(&db, &book.id, &ids.1).await.unwrap();
    let img = match img {
        Some(img) => img,
        None => return HttpResponse::NotFound().body("Image not found"),
    };

    let data = images::get_image_data(&db, &img.image_id).await.unwrap();
    let data = match data {
        Some(data) => data,
        None => return HttpResponse::NotFound().body("Data not found"),
    };

    HttpResponse::Ok().content_type(data.mime).append_header(("Cache-Control", "max-age=604800")).body(data.content)
}