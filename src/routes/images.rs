use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};

use crate::database::images::get_image_data_by_image_id;
use crate::database::Database;

#[get("/{id}")]
pub async fn get_image(db: Data<Database>, path: Path<String>) -> HttpResponse {
    let image = match get_image_data_by_image_id(&db, &path.into_inner()).await {
        Some(image) => image,
        None => return HttpResponse::NotFound().body("Image not found"),
    };

    HttpResponse::Ok()
        .insert_header(("Cache-Control", "max-age=2630000, no-transform"))
        .content_type(image.mime)
        .body(image.content)
}
