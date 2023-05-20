use actix_web::web::{self, ServiceConfig};

mod books;
mod collections;

pub fn init_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/books")
            .service(books::from_epub)
            .service(books::get_book)
            .service(books::get_book_image),
    )
    .service(
        web::scope("/collections")
            .service(collections::create_collection)
            .service(collections::get_collection)
            .service(collections::get_collection_thumbnail)
            .service(collections::set_collection_thumbnail)
            .service(collections::get_collections),
    );
}
