use ntex::web;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}
