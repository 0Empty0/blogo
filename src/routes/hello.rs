use crate::handlers::hello;
use ntex::web::ServiceConfig;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(hello::hello);
}
