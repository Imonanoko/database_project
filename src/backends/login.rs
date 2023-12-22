use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;

// 结构体用于反序列化 POST 请求的 JSON 数据
#[derive(Deserialize, Debug)]
pub struct LoginData {
    // 改成post要得name
    id: String,
    phone: String,
}
pub async fn login_verification(
    data: web::Json<LoginData>,
    db: web::Data<Pool>,
    session: Session,
) -> HttpResponse {
    let mut conn = match db.get_conn() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    // 讀取前端的資料
    let _id = &data.id;
    let _phone = &data.phone;
    let result: Vec<(String, String, bool)> = conn
        .exec_map(
            "SELECT * FROM login_data WHERE id_number = :id AND phone_number = :phone",
            params! {
                "id" => _id,
                "phone" => _phone
            },
            |row: mysql::Row| {
                let id_number: String = row.get("id_number").unwrap();
                let phone_number: String = row.get("phone_number").unwrap();
                let is_admin: bool = row.get("is_admin").unwrap();
                (id_number, phone_number, is_admin)
            },
        )
        .expect("Query failed.");
    if !result.is_empty() {
        let first_row = &result[0];
        // println!("first_row {:?}",first_row);
        let (id_number, phone_number, is_admin) = first_row;
        // println!("id_number: {}, phone_number: {}, is_admin: {}", id_number, phone_number, is_admin);
        session.insert("id", id_number).expect("id set failed");
        session
            .insert("phone", phone_number)
            .expect("phone set failed");
        session
            .insert("is_admin", is_admin)
            .expect("is_admin set failed");
        let response_body = serde_json::json!({
            "login_flag": "true",
        });
        HttpResponse::Ok()
            .content_type("application/json")
            .json(response_body)
    } else {
        let response_body = serde_json::json!({
            "login_flag": "false",
        });
        HttpResponse::Ok()
            .content_type("application/json")
            .json(response_body)
    }
}
