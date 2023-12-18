use crate::http_status::{error_401, error_handler};
use crate::jump_page::verification_session;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct SendData {
    customer_count: i32,
    average_age: f32,
    disabled_customers: i32,
    active_customers: i32,
}

pub async fn client_data(session: Session, db: web::Data<Pool>) -> HttpResponse {
    if verification_session(&session) {
        let mut conn = match db.get_conn() {
            Ok(conn) => conn,
            Err(_) => return error_handler().await,
        };
        let customer_count: i32 = match conn.exec_first("SELECT COUNT(*) FROM customer_data", ()) {
            Ok(Some(customer_count)) => customer_count,
            Ok(None) => 0,
            Err(_e) => return error_handler().await,
        };

        let average_age: f32 = match conn.exec_first("SELECT AVG(age) FROM customer_data", ()) {
            Ok(Some(average_age)) => average_age,
            Ok(None) => 0.0,
            Err(_e) => return error_handler().await,
        };

        let active_customers: i32 = match conn.exec_first(
            "SELECT COUNT(*) FROM customer_data WHERE status = '正常'",
            (),
        ) {
            Ok(Some(active_customers)) => active_customers,
            Ok(None) => 0,
            Err(_e) => return error_handler().await,
        };

        let disabled_customers: i32 = match conn.exec_first(
            "SELECT COUNT(*) FROM customer_data WHERE status = '停用'",
            (),
        ) {
            Ok(Some(disabled_customers)) => disabled_customers,
            Ok(None) => 0,
            Err(_e) => return error_handler().await,
        };

        let data = SendData {
            customer_count,
            average_age,
            active_customers,
            disabled_customers,
        };
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(data);
    }
    return error_401().await;
}
