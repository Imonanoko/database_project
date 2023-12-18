use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct Data {
    startdate: String,
    enddate: String,
}
#[derive(Serialize, Debug)]
pub struct SendData {
    name: String,
    id_number: String,
    money: f32,
}
pub async fn week_allclient_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    let start_date: String = data
        .startdate
        .split('T')
        .next()
        .unwrap_or_default()
        .to_string()
        .replace("/", "-");
    let end_date: String = data
        .enddate
        .split('T')
        .next()
        .unwrap_or_default()
        .to_string()
        .replace("/", "-");
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
                             WHERE o.order_date BETWEEN :startdate AND :enddate
                             GROUP BY o.id_number;
                             ",
                         params! {
                             "startdate"=>&start_date,
                             "enddate"=>&end_date,
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
                        "results":result,
                        "startdate":start_date,
                        "enddate":end_date
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
