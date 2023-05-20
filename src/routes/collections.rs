use std::io::Read;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{web::{Json, Data, self}, HttpResponse, post, get};
use cuid2::cuid;
use serde_json::json;
use crate::database::{collections::{Collection, self}, Database, images::{self, ImageData}, books};

#[post("/create")]
pub async fn create_collection(db: Data<Database>, mut collection: Json<Collection>) -> HttpResponse{
    collection.id = cuid();
    collections::create_collection(&db, &collection).await.unwrap();
    HttpResponse::Created().json(collection)
}

#[get("/{id}/thumbnail")]
pub async fn get_collection_thumbnail(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let collection = collections::get_collection(&db, &id).await.unwrap().unwrap();
    if let Some(thumbnail_id) = collection.thumbnail_id {
        let thumbnail = images::get_image_data(&db, &thumbnail_id).await.unwrap().unwrap();
        HttpResponse::Ok().content_type(thumbnail.mime).body(thumbnail.content)
    } else {
        HttpResponse::NotFound().body("Collection has no thumbnail")
    }
}


#[derive(MultipartForm)]
pub struct SetThumbnailRequest {
    pub thumbnail: TempFile,
}
#[post("/{id}/thumbnail")]
pub async fn set_collection_thumbnail(db: Data<Database>, id: web::Path<String>, thumb: MultipartForm<SetThumbnailRequest>) -> HttpResponse {
    let collection = collections::get_collection(&db, &id).await.unwrap();
    let collection = match collection {
        Some(collection) => collection,
        None => return HttpResponse::NotFound().body("Collection not found")
    };

    let mut buff = Vec::new();
    thumb.thumbnail.file.as_file().read_to_end(&mut buff).unwrap();

    let thumbnail_id = images::create_image_data(&db, &ImageData {
        id: 0,
        mime: thumb.thumbnail.content_type.clone().unwrap().to_string(),
        content: buff,

    }).await.unwrap();

    collections::set_collection_thumbnail(&db, &collection.id, &thumbnail_id).await.unwrap();
    HttpResponse::Ok().json(json!({
        "id": thumbnail_id
    }))
}

#[get("/{id}")]
pub async fn get_collection(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let collection = collections::get_collection(&db, &id).await.unwrap();
    if let Some(collection) = collection {
        let books = books::get_books_by_collection(&db, &collection.id).await.unwrap();

        HttpResponse::Ok().json(json!({
            "collection": collection,
            "books": books
        }))
    } else {
        HttpResponse::NotFound().body("Collection not found")
    }
}

#[get("/")]
pub async fn get_collections(db: Data<Database>) -> HttpResponse {
    let collections = collections::get_collections(&db).await.unwrap();
    HttpResponse::Ok().json(collections)
}