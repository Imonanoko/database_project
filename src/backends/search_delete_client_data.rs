use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
}
pub async fn search_delete_client_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                let mut conn = match db.get_conn() {
                    Ok(conn) => conn,
                    Err(_) => return error_handler().await,
                };
                let useornot: String = match conn.exec_first::<String, _, _>(
                    "SELECT status FROM customer_data WHERE id_number = :id_number",
                    params! {
                        "id_number" => data.customer_id.clone()
                    },
                ) {
                    Ok(Some(status)) => status,
                    Ok(None) => "".to_string(),
                    Err(_e) => {
                        eprintln!("Database query error: {:?}", _e);
                        return error_handler().await;
                    }
                };
                let response_body = serde_json::json!({
                    "status": "true",
                    "useornot":useornot,
                });
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(response_body);
            } else {
                return HttpResponse::Ok().body("死小孩不要亂戳");
            }
        }
        Ok(None) => {
            return error_401().await;
        }
        Err(_e) => {
            return error_handler().await;
        }
    }
}
