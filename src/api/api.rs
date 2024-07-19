use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Function to start the API server
pub async fn api(ip: &str) {
    println!("Api Started on ip {}", ip);
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(hello_get))
    })
    .bind(ip)
    .unwrap()
    .run()
    .await
    .unwrap();
}

// Handler for the /hello route
async fn hello_get() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}
