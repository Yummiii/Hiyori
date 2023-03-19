use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use actix_web::{delete, get, post};
use cuid2::cuid;
use serde::Deserialize;
use serde_json::json;

use crate::database::books::get_books_by_collection;
use crate::database::collections::Collection;
use crate::database::images::{delete_image_data, get_images_by_book};
use crate::database::{books, collections, Database};

#[derive(Deserialize)]
pub struct CreateCollectionRequest {
    pub name: String,
}

#[post("/create")]
pub async fn create_collection(
    database: Data<Database>,
    collection: Json<CreateCollectionRequest>,
) -> HttpResponse {
    let collection = Collection {
        name: collection.name.clone(),
        id: cuid(),
        thumb: None,
    };
    collections::create_collection(&database, &collection)
        .await
        .unwrap();

    HttpResponse::Created().json(collection)
}

#[get("")]
pub async fn get_collections(database: Data<Database>) -> HttpResponse {
    let collections = collections::get_collections(&database).await.unwrap();
    let collections = collections
        .into_iter()
        .map(|x| {
            json!({
                "id": x.id,
                "name": x.name,
                "has_thumb": x.thumb.is_some(),
            })
        })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(collections)
}

#[get("/{id}/books")]
pub async fn get_collection_books(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let collection = match collections::get_collection(&db, &id).await {
        Some(collection) => collection,
        None => return HttpResponse::NotFound().body("Collection not found"),
    };

    let books = get_books_by_collection(&db, &collection.id).await;
    HttpResponse::Ok().json(books)
}

#[delete("/{id}")]
pub async fn delete_collection(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let collection = match collections::get_collection(&db, &id).await {
        Some(collection) => collection,
        None => return HttpResponse::NotFound().body("Collection not found"),
    };

    for book in get_books_by_collection(&db, &collection.id).await {
        let images = get_images_by_book(&db, &book.id).await;
        books::delete_book(&db, &book.id).await.unwrap();

        for image in images {
            delete_image_data(&db, &image.data).await.unwrap();
        }
    }

    collections::delete_collection(&db, &collection.id)
        .await
        .unwrap();
    HttpResponse::Ok().body("Collection deleted")
}
