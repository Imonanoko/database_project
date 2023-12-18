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

#[derive(Debug, Serialize)]
pub struct SendData {
    name: String,
    phone: String,
    total_order_money: f32,
}
pub async fn week_all_client_money_all_data(
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
                let result: Vec<SendData> = conn
                        .exec_map(
                        "SELECT c.customer_name, c.phone_number, COALESCE(SUM(o.order_amount), 0.0) AS total_order_amount \
                        FROM customer_data c \
                        LEFT JOIN order_records o ON c.id_number = o.id_number \
                        WHERE o.order_date BETWEEN :start_date AND :end_date \
                        GROUP BY c.id_number, c.phone_number",
                        params! {
                            "start_date" => start_date,
                            "end_date" => end_date,
                        },
                        |row: mysql::Row| {
                            let name: String = row.get("customer_name").unwrap();
                            let phone: String = row.get("phone_number").unwrap();
                            let total_order_money: f32 = row.get("total_order_amount").unwrap();
                            SendData {
                                name,
                                phone,
                                total_order_money,
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
