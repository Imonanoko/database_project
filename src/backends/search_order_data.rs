use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
//接收資料要反序列化
#[derive(Deserialize, Debug)]
pub struct Data {
    id: String,
}
//送出資料要反序列化
#[derive(Debug, Serialize)]
pub struct SendData {
    order_date: NaiveDate,
    expected_delivery_date: NaiveDate,
    actual_delivery_date: NaiveDate,
    product_name: String,
    unit: String,
    quantity: i32,
    unit_price: f32,
    order_amount: f32,
    supplier_name: String,
    supplier_id: String,
}
pub async fn search_order_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    let id = &data.id;
    let mut conn = match db.get_conn() {
        Ok(conn) => conn,
        Err(_) => return error_handler().await,
    };
    let session_id = session.get::<String>("id").expect("no session id");
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                let result: Vec<SendData> = conn
                    .exec_map(
                        "SELECT * FROM order_records WHERE id_number = :id ",
                        params! {
                            "id" => id
                        },
                        |row: mysql::Row| {
                            let order_date: NaiveDate = row.get("order_date").unwrap();
                            let expected_delivery_date: NaiveDate =
                                row.get("expected_delivery_date").unwrap();
                            let actual_delivery_date: NaiveDate =
                                row.get("actual_delivery_date").unwrap();
                            let product_name: String = row.get("product_name").unwrap();
                            let unit: String = row.get("unit").unwrap();
                            let quantity: i32 = row.get("quantity").unwrap();
                            let unit_price: f32 = row.get("unit_price").unwrap();
                            let order_amount: f32 = row.get("order_amount").unwrap();
                            let supplier_name: String = row.get("supplier_name").unwrap();
                            let supplier_id: String = row.get("supplier_id").unwrap();
                            SendData {
                                order_date: order_date,
                                expected_delivery_date: expected_delivery_date,
                                actual_delivery_date: actual_delivery_date,
                                product_name: product_name,
                                unit: unit,
                                quantity: quantity,
                                unit_price: unit_price,
                                order_amount: order_amount,
                                supplier_name: supplier_name,
                                supplier_id: supplier_id,
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
                                "SELECT * FROM order_records WHERE id_number = :id ",
                                params! {
                                    "id" => id
                                },
                                |row: mysql::Row| {
                                    let order_date: NaiveDate = row.get("order_date").unwrap();
                                    let expected_delivery_date: NaiveDate =
                                        row.get("expected_delivery_date").unwrap();
                                    let actual_delivery_date: NaiveDate =
                                        row.get("actual_delivery_date").unwrap();
                                    let product_name: String = row.get("product_name").unwrap();
                                    let unit: String = row.get("unit").unwrap();
                                    let quantity: i32 = row.get("quantity").unwrap();
                                    let unit_price: f32 = row.get("unit_price").unwrap();
                                    let order_amount: f32 = row.get("order_amount").unwrap();
                                    let supplier_name: String = row.get("supplier_name").unwrap();
                                    let supplier_id: String = row.get("supplier_id").unwrap();
                                    SendData {
                                        order_date: order_date,
                                        expected_delivery_date: expected_delivery_date,
                                        actual_delivery_date: actual_delivery_date,
                                        product_name: product_name,
                                        unit: unit,
                                        quantity: quantity,
                                        unit_price: unit_price,
                                        order_amount: order_amount,
                                        supplier_name: supplier_name,
                                        supplier_id: supplier_id,
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
