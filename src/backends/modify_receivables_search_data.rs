use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
}
#[derive(Serialize, Debug)]
pub struct SendData {
    name: String,
    money: f32,
    date: NaiveDate,
    hastenmoney: f32,
}
pub async fn modify_receivables_search_data(
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
                let result: Vec<SendData> = conn
                    .exec_map(
                        "SELECT 
                        CustomerName, 
                        ReceivableAmount, 
                        DueDate, 
                        PendingCollectionAmount
                    FROM AccountsReceivable
                    WHERE IDNumber = :id_number;",
                        params! {
                            "id_number" => &data.customer_id,
                        },
                        |row: mysql::Row| SendData {
                            name: row.get("CustomerName").unwrap(),
                            money: row.get("ReceivableAmount").unwrap(),
                            date: row.get("DueDate").unwrap(),
                            hastenmoney: row.get("PendingCollectionAmount").unwrap(),
                        },
                    )
                    .expect("Query failed.");
                if !result.is_empty() {
                    let response_body = serde_json::json!({
                        "status": "true",
                        "results":result,
                    });
                    return HttpResponse::Ok()
                        .content_type("application/json")
                        .json(response_body);
                } else {
                    let response_body = serde_json::json!({
                        "status": "duplicate_name",
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
