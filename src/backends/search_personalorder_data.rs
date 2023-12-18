use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    start_date: String, // 修正这一行
    end_date: String,
    id: String,
}
#[derive(Debug, Serialize)]
pub struct SendData {
    id: String,
    total_order_money: f32,
}
pub async fn search_personalorder_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    let start_date = &data.start_date.replace("/", "-");
    let end_date = &data.end_date.replace("/", "-");
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
                        "SELECT COALESCE(SUM(order_amount), 0.0) AS total_order_amount \
                        FROM order_records \
                        WHERE order_date BETWEEN :start_date AND :end_date AND id_number = :id_number ",
                        params! {
                            "start_date" => start_date,
                            "end_date" => end_date,
                            "id_number" => id,
                        },
                        |row: mysql::Row| {
                            let total_order_money:f32 = row.get("total_order_amount").unwrap();
                            SendData {
                                id:id.to_string(),
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
                if let Some(sid) = session_id {
                    if id == sid.as_str() {
                        let result: Vec<SendData> = conn
                        .exec_map(
                            "SELECT COALESCE(SUM(order_amount), 0.0) AS total_order_amount \
                            FROM order_records \
                            WHERE order_date BETWEEN :start_date AND :end_date AND id_number = :id_number ",
                            params! {
                                "start_date" => start_date,
                                "end_date" => end_date,
                                "id_number" => id,
                            },
                            |row: mysql::Row| {
                                let total_order_money:f32 = row.get("total_order_amount").unwrap();
                                SendData {
                                    id:id.to_string(),
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
