use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct Data {
    start_date: String,
    end_date: String,
}
#[derive(Serialize, Debug)]
pub struct SendData {
    name: String,
    total_order_amount: f32,
}
pub async fn week_client_money_all_data(
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
                        "SELECT c.customer_name, SUM(o.order_amount) AS total_order_amount
                                FROM order_records o
                                JOIN customer_data c ON o.id_number = c.id_number
                                WHERE o.order_date BETWEEN :startdate AND :enddate
                                GROUP BY o.id_number;
                                ",
                        params! {
                            "startdate"=>&data.start_date,
                            "enddate"=>&data.end_date,
                        },
                        |row: mysql::Row| {
                            let name = row.get("customer_name").unwrap();
                            let total_order_amount = row.get("total_order_amount").unwrap();
                            SendData {
                                name,
                                total_order_amount,
                            }
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
