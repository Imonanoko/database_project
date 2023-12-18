use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    start_date: String, // 修正这一行
    end_date: String,
}
pub async fn search_allamountorder_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                let start_date = &data.start_date.replace("/", "-");
                let end_date = &data.end_date.replace("/", "-");
                let mut conn = match db.get_conn() {
                    Ok(conn) => conn,
                    Err(_) => return error_handler().await,
                };
                let result: Vec<f32> = conn
                    .exec_map(
                        "SELECT COALESCE(SUM(order_amount), 0.0) AS total_order_amount \
                        FROM order_records \
                        WHERE order_date BETWEEN :start_date AND :end_date ",
                        params! {
                            "start_date" => start_date,
                            "end_date" => end_date,
                        },
                        |row: mysql::Row| {
                            let total_order_money: f32 = row.get("total_order_amount").unwrap();
                            total_order_money
                        },
                    )
                    .expect("Query failed.");
                if !result.is_empty() {
                    let response_body = serde_json::json!({
                        "status": "true",
                        "total_order_amount":result[0],
                    });
                    return HttpResponse::Ok()
                        .content_type("application/json")
                        .json(response_body);
                } else {
                    let response_body = serde_json::json!({
                        "status": "empty",
                    });
                    return HttpResponse::Ok()
                        .content_type("application/json")
                        .json(response_body);
                }
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
