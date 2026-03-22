use crate::models::{User, Post, CreateUserRequest};
use crate::services::{UserService, PostService};
use actix_web::{web, HttpResponse};
use std::sync::Mutex;

pub struct AppState {
    pub user_service: Mutex<UserService>,
    pub post_service: Mutex<PostService>,
}

pub async fn get_users(state: web::Data<AppState>) -> HttpResponse {
    let service = state.user_service.lock().unwrap();
    match service.get_all_users() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalError().body(e.to_string()),
    }
}

pub async fn get_user_by_id(
    state: web::Data<AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let id = path.into_inner();
    let service = state.user_service.lock().unwrap();
    
    match service.get_user_by_id(id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json("User not found"),
    }
}

pub async fn create_user(
    state: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let service = state.user_service.lock().unwrap();
    
    match service.create_user(body.into_inner()) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub async fn get_posts(state: web::Data<AppState>) -> HttpResponse {
    let service = state.post_service.lock().unwrap();
    match service.get_all_posts() {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => HttpResponse::InternalError().body(e.to_string()),
    }
}

pub async fn get_users_v2(state: web::Data<AppState>) -> HttpResponse {
    let service = state.user_service.lock().unwrap();
    match service.get_all_users() {
        Ok(users) => HttpResponse::Ok().json(serde_json::json!({
            "version": "v2",
            "users": users
        })),
        Err(e) => HttpResponse::InternalError().body(e.to_string()),
    }
}
