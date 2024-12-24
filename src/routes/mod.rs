pub mod hello;

use ntex::web::ServiceConfig;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    hello::configure_routes(cfg);
}
