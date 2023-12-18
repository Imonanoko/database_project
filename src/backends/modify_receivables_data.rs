use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
    receivable_money: f32,
    hasten_money: f32,
    receivable_date: String,
}
pub async fn modify_receivables_data(
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
                let params = params! {
                    "receivable_amount" => data.receivable_money,
                    "due_date" => data.receivable_date.clone().replace("/", "-"),
                    "pending_collection_amount" => data.hasten_money,
                    "id"=>&data.customer_id,
                };
                let query = "UPDATE AccountsReceivable
                SET ReceivableAmount = :receivable_amount, DueDate = :due_date, PendingCollectionAmount = :pending_collection_amount
                WHERE IDNumber = :id";
                match conn.exec_drop(query, params) {
                    Ok(_) => {
                        let response_body = serde_json::json!({
                            "create_flag": "true",
                        });
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(response_body);
                    }
                    Err(_e) => {
                        return error_handler().await;
                    }
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
