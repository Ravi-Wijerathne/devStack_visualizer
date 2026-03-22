use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

mod handlers;
mod models;
mod services;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some("OK"),
        error: None,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/api/users", web::get().to(handlers::get_users))
            .route("/api/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/api/users", web::post().to(handlers::create_user))
            .route("/api/posts", web::get().to(handlers::get_posts))
            .service(web::scope("/api/v2")
                .route("/users", web::get().to(handlers::get_users_v2))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
