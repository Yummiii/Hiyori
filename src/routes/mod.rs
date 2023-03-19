use actix_web::web::{self, ServiceConfig};

mod books;
mod collections;
mod images;

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/collections")
            .service(collections::create_collection)
            .service(collections::get_collections)
            .service(collections::get_collection_books)
            .service(collections::delete_collection),
    )
    .service(
        web::scope("/books")
            .service(books::from_epub)
            .service(books::get_cover)
            .service(books::get_book)
            .service(books::delete_book),
    )
    .service(web::scope("images").service(images::get_image));
}
