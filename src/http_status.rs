use actix_web::HttpResponse;
//各種錯誤
pub async fn error_handler() -> HttpResponse {
    HttpResponse::InternalServerError().body("Internal Server Error")
}

pub async fn error_404() -> HttpResponse {
    HttpResponse::NotFound().body(include_str!("html/404.html"))
}
pub async fn error_401() -> HttpResponse {
    HttpResponse::Unauthorized().body("Unauthorized")
}
pub async fn permission_denied() -> HttpResponse {
    HttpResponse::Forbidden().body(include_str!("html/permission_denied.html"))
}
