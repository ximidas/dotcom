use actix_web::web;

pub fn initialize(cfg: &mut web::ServiceConfig) {
    crate::routes::api::setup_routes(cfg);
}
