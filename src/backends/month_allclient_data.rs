use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct Data {
    year: String,
    month: String,
}
#[derive(Serialize, Debug)]
pub struct SendData {
    name: String,
    id_number: String,
    money: f32,
}
pub async fn month_allclient_data(
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
                            "SELECT o.id_number, c.customer_name, SUM(o.order_amount) AS total_order_amount
                            FROM order_records o
                            JOIN customer_data c ON o.id_number = c.id_number
                            WHERE YEAR(o.order_date) = :year AND MONTH(o.order_date) = :month
                            GROUP BY o.id_number;
                            ",
                        params! {
                            "year"=>&data.year,
                            "month"=>&data.month,
                        },
                        |row: mysql::Row| {
                                let name = row.get("customer_name").unwrap();
                                let id_number = row.get("id_number").unwrap();
                                let money = row.get("total_order_amount").unwrap();
                            SendData {
                                name,
                                id_number,
                                money,
                            }
                        },
                    )
                    .expect("Query failed.");
                if !result.is_empty() {
                    let response_body = serde_json::json!({
                        "status": "true",
                        "results":result
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
