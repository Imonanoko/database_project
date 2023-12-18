use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
}
#[derive(Debug, Serialize)]
pub struct SendData {
    address: String,
    phone: String,
    name: String,
    age: i32,
    occupation: String,
    registration_date: NaiveDate,
    useornot: String,
}
pub async fn modify_client_search_data(
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
                            address, phone_number, customer_name, age, occupation, registration_date, status
                            FROM customer_data
                            WHERE id_number = :id;",
                        params! {
                            "id" => &data.customer_id,
                        },
                        |row: mysql::Row| {
                            let address: String = row.get("address").unwrap();
                            let phone: String = row.get("phone_number").unwrap();
                            let name: String = row.get("customer_name").unwrap();
                            let age: i32 = row.get("age").unwrap();
                            let occupation: String = row.get("occupation").unwrap();
                            let registration_date: NaiveDate = row.get("registration_date").unwrap();
                            let useornot: String = row.get("status").unwrap();
                            SendData {
                                address,
                                phone,
                                name,
                                age,
                                occupation,
                                registration_date,
                                useornot,
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
