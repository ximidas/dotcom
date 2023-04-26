use actix_web::web;

pub fn setup_routes(cfg: &mut web::ServiceConfig) -> &mut web::ServiceConfig {
    cfg.service(crate::handlers::post_handler::total_pages)
        .service(crate::handlers::post_handler::get_all)
        .service(crate::handlers::post_handler::get_by_id)
}
