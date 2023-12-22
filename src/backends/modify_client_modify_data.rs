use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use crate::backends::library::error_msg;

#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
    address: String,
    phone: String,
}
pub async fn modify_client_modify_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                if data.phone.len()!=10 {
                    return error_msg("電話號碼應該為十碼").await;
                }
                if data.address.len() == 0 {
                    return error_msg("地址不能為空").await;
                }
                if data.address.len() > 30 {
                    return error_msg("地址不能超過30字").await;
                }
                let mut conn = match db.get_conn() {
                    Ok(conn) => conn,
                    Err(_) => return error_handler().await,
                };
                let customer_data_params = params! {
                    "customer_id" => &data.customer_id,
                    "address" => &data.address,
                    "phone" => &data.phone,
                };
                let customer_data_query = "UPDATE customer_data
                SET address = :address, phone_number = :phone
                WHERE id_number = :customer_id";
                match conn.exec_drop(customer_data_query, customer_data_params) {
                    Ok(_) => {
                        let login_data_params = params! {
                            "customer_id" => &data.customer_id,
                            "phone" => &data.phone,
                        };
                        let login_data_query = "UPDATE login_data
                        SET phone_number = :phone
                        WHERE id_number = :customer_id";
                        match conn.exec_drop(login_data_query, login_data_params) {
                            Ok(_) => {
                                let response_body = serde_json::json!({
                                    "status": "true",
                                });
                                return HttpResponse::Ok()
                                    .content_type("application/json")
                                    .json(response_body);
                            }
                            Err(_e) => {
                                let response_body = serde_json::json!({
                                    "status": "false",
                                });
                                return HttpResponse::Ok()
                                    .content_type("application/json")
                                    .json(response_body);
                            }
                        }
                    }
                    Err(_e) => {
                        let response_body = serde_json::json!({
                            "status": "false",
                        });
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(response_body);
                    }
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
