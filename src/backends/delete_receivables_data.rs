use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
}
pub async fn delete_receivables_data(
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
                let exist: bool = match conn.exec_first::<String, _, _>(
                    "SELECT IDNumber FROM AccountsReceivable WHERE IDNumber = :id_number",
                    params! {
                        "id_number" => &data.customer_id
                    },
                ) {
                    Ok(Some(_id)) => true,
                    Ok(None) => false,
                    Err(_e) => {
                        eprintln!("Database query error: {:?}", _e);
                        return error_handler().await;
                    }
                };
                if exist {
                    let delete_query =
                        "UPDATE AccountsReceivable SET status='未繳費' WHERE IDNumber = :id_number";
                    let delete_params = params! {
                        "id_number"=>data.customer_id.clone(),
                    };
                    match conn.exec_drop(delete_query, delete_params) {
                        Ok(_) => {
                            let response_body = serde_json::json!({
                                "create_flag": "true",
                            });
                            return HttpResponse::Ok()
                                .content_type("application/json")
                                .json(response_body);
                        }
                        Err(_e) => {
                            return error_handler().await;
                        }
                    }
                } else {
                    let response_body = serde_json::json!({
                        "create_flag": "false",
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
