use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, HttpResponse};
use cuid2::cuid;
use epub::doc::EpubDoc;
use itertools::Itertools;
use serde_json::json;

use crate::database::books::{create_book, Book};
use crate::database::collections::get_collection;
use crate::database::images::{
    create_image, create_image_data, delete_image_data, get_image_data, get_images_by_book, Image,
    ImageData,
};
use crate::database::{books, Database};

#[derive(MultipartForm)]
pub struct FromEpubRequest {
    pub epub: TempFile,
    pub collection: Text<String>,
}

#[post("/from_epub")]
pub async fn from_epub(db: Data<Database>, epub: MultipartForm<FromEpubRequest>) -> HttpResponse {
    //Ths could be way better, but it works for now. I don't pride myself on my rust skills.
    if let Some(content_type) = &epub.epub.content_type {
        if content_type.to_string() != "application/epub+zip" {
            return HttpResponse::BadRequest().body("File is not an epub");
        }
    }

    let collection = epub.collection.0.clone();
    let collection = if get_collection(&db, &collection).await.is_none() {
        return HttpResponse::BadRequest().body("Collection does not exist");
    } else {
        collection
    };

    let mut doc = EpubDoc::from_reader(epub.epub.file.as_file()).unwrap();

    let cover = doc.get_cover().unwrap();
    let cover = create_image_data(
        &db,
        &ImageData {
            id: 0,
            mime: cover.1,
            content: cover.0,
        },
    )
    .await
    .unwrap();

    let book = Book {
        id: cuid(),
        title: doc.mdata("title").unwrap(),
        cover,
        collection,
    };
    create_book(&db, &book).await.unwrap();

    let mut i = 0;
    let resources = doc.resources.clone();
    for res in resources.iter().sorted() {
        if res.1 .1 == "image/jpeg" || res.1 .1 == "image/png" {
            let img = doc.get_resource_by_path(&res.1 .0).unwrap();
            let img = if res.0 == "x_cover" {
                cover
            } else {
                create_image_data(
                    &db,
                    &ImageData {
                        id: 0,
                        mime: res.1 .1.clone(),
                        content: img,
                    },
                )
                .await
                .unwrap()
            };

            create_image(
                &db,
                &Image {
                    id: cuid(),
                    book: book.id.clone(),
                    page: i,
                    data: img,
                },
            )
            .await
            .unwrap();

            i += 1;
        }
    }

    HttpResponse::Created().json(book)
}

#[get("/{id}/cover")]
pub async fn get_cover(database: Data<Database>, id: Path<String>) -> HttpResponse {
    let book = if let Some(book) = books::get_book(&database, &id).await {
        book
    } else {
        return HttpResponse::NotFound().body("Book not found");
    };

    let data = if let Some(data) = get_image_data(&database, book.cover).await {
        data
    } else {
        return HttpResponse::NotFound().body("Book cover not found");
    };

    HttpResponse::Ok()
        .content_type(data.mime)
        .body(data.content)
}

#[get("/{id}")]
pub async fn get_book(database: Data<Database>, id: Path<String>) -> HttpResponse {
    let book = if let Some(book) = books::get_book(&database, &id).await {
        book
    } else {
        return HttpResponse::NotFound().body("Book not found");
    };

    let images = get_images_by_book(&database, &book.id).await;

    HttpResponse::Ok().json(json!({
        "book": book,
        "images": images
    }))
}

#[delete("/{id}")]
pub async fn delete_book(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let book = if let Some(book) = books::get_book(&db, &id).await {
        book
    } else {
        return HttpResponse::NotFound().body("Book not found");
    };

    let images = get_images_by_book(&db, &book.id).await;
    books::delete_book(&db, &book.id).await.unwrap();

    for image in images {
        delete_image_data(&db, &image.data).await.unwrap();
    }
    HttpResponse::Ok().body("Book deleted")
}
