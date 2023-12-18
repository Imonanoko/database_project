use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use base64::encode;
use chrono::NaiveDate;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
}
#[derive(Debug, Serialize)]
pub struct SendData {
    customer_name: String,
    id_number: String,
    phone_number: String,
    address: String,
    age: i32,
    occupation: String,
    registration_date: NaiveDate,
    photo_base64: String,
    status: String,
}
pub async fn search_client_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    let id = &data.customer_id;
    let session_id = session.get::<String>("id").expect("no session id");
    let mut conn = match db.get_conn() {
        Ok(conn) => conn,
        Err(_) => return error_handler().await,
    };
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                let result: Vec<SendData> = conn
                    .exec_map(
                        "SELECT * FROM customer_data WHERE id_number = :id ",
                        params! {
                            "id" => &data.customer_id,
                        },
                        |row: mysql::Row| {
                            let customer_name = row.get("customer_name").unwrap();
                            let id_number = row.get("id_number").unwrap();
                            let phone_number = row.get("phone_number").unwrap();
                            let age = row.get("age").unwrap();
                            let occupation = row.get("occupation").unwrap();
                            let photo: Vec<u8> = row.get("photo").unwrap();
                            let registration_date = row.get("registration_date").unwrap();
                            let status = row.get("status").unwrap();
                            let address = row.get("address").unwrap();
                            let photo_base64 = encode(photo);
                            SendData {
                                customer_name,
                                id_number,
                                phone_number,
                                age,
                                address,
                                occupation,
                                registration_date,
                                photo_base64,
                                status,
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
                if let Some(sid) = session_id {
                    if id == sid.as_str() {
                        let result: Vec<SendData> = conn
                            .exec_map(
                                "SELECT * FROM customer_data WHERE id_number = :id ",
                                params! {
                                    "id" => &data.customer_id,
                                },
                                |row: mysql::Row| {
                                    let customer_name = row.get("customer_name").unwrap();
                                    let id_number = row.get("id_number").unwrap();
                                    let phone_number = row.get("phone_number").unwrap();
                                    let age = row.get("age").unwrap();
                                    let occupation = row.get("occupation").unwrap();
                                    let photo: Vec<u8> = row.get("photo").unwrap();
                                    let registration_date = row.get("registration_date").unwrap();
                                    let status = row.get("status").unwrap();
                                    let address = row.get("address").unwrap();
                                    let photo_base64 = encode(photo);
                                    SendData {
                                        customer_name,
                                        id_number,
                                        phone_number,
                                        age,
                                        address,
                                        occupation,
                                        registration_date,
                                        photo_base64,
                                        status,
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
                        let response_body = serde_json::json!({
                            "status": "permission denied",
                        });
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(response_body);
                    }
                } else {
                    return error_handler().await;
                }
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
