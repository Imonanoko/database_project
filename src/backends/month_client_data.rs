use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Data {
    year: String,
    month: String,
}
pub async fn month_client_data(
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
                let totalmoney: f32 = match conn.exec_first::<f32, _, _>(
                    "SELECT  SUM(order_amount) AS total_order_amount
                            FROM order_records
                            WHERE YEAR(order_date) = :year AND MONTH(order_date) = :month
                            ",
                    params! {
                        "year"=>&data.year,
                        "month"=>&data.month,
                    },
                ) {
                    Ok(Some(totalmoney)) => totalmoney,
                    Ok(None) => 0.0,
                    Err(_e) => {
                        eprintln!("Database query error: {:?}", _e);
                        return error_handler().await;
                    }
                };
                if totalmoney > 0.0 {
                    let response_body = serde_json::json!({
                        "status":"true",
                        "year":data.year,
                        "month":data.month,
                        "totalmoney":totalmoney
                    });
                    return HttpResponse::Ok()
                        .content_type("application/json")
                        .json(response_body);
                } else {
                    let response_body = serde_json::json!({
                        "status":"empty",
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
