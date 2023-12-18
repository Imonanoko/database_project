use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
//照片問題待解決
#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
    money: f32,
    date: String,
    hastenmoney: f32,
    name: String,
}
pub async fn add_receivables_data(
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

                // Executing the insertion query
                let query = "INSERT INTO AccountsReceivable
                (CustomerName, IDNumber, ReceivableAmount, DueDate, PendingCollectionAmount)
                VALUES
                (?, ?, ?, ?, ?)";

                let params = (
                    &data.name,
                    &data.customer_id,
                    &data.money,
                    &data.date,
                    &data.hastenmoney,
                );
                match conn.exec_drop(query, params) {
                    Ok(_) => {
                        // If the query was successful
                        let response_body = serde_json::json!({
                            "create_flag": "true",
                        });
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(response_body);
                    }
                    Err(_err) => {
                        println!("{}", _err);
                        let response_body = serde_json::json!({
                            "create_flag": "false",
                        });
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(response_body);
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
