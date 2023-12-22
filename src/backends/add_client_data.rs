use crate::http_status::{error_401, error_handler};
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
//需要再main裡面先引入
use crate::backends::library::{error_msg,date_check};

#[derive(Deserialize, Debug)]
pub struct Data {
    name: String,
    customer_id: String,
    phone: String,
    age: i32,
    address: String,
    occupation: String,
    add_date: String,
    userornot: String,
}
pub async fn add_client_data(
    mut payload: Multipart,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                let mut data = Data {
                    name: String::new(),
                    customer_id: String::new(),
                    phone: String::new(),
                    age: 0,
                    address: String::new(),
                    occupation: String::new(),
                    add_date: String::new(),
                    userornot: String::new(),
                };
                let mut photo: Vec<u8> = Vec::new();
                while let Ok(Some(mut field)) = payload.try_next().await {
                    let content_disposition = field.content_disposition();
                    let field_name = content_disposition
                        .get_name()
                        .map(|s| s.to_string())
                        .unwrap_or_default();

                    match field_name.as_str() {
                        "picture" => {
                            while let Some(chunk) = field.next().await {
                                let data_chunk = chunk.expect("chunk failed");
                                photo.extend_from_slice(&data_chunk);
                            }
                        }
                        _ => {
                            if let Some(chunk) = field.next().await {
                                let data_chunk = chunk.expect("chunk failed");
                                let value_str = String::from_utf8(data_chunk.to_vec()).unwrap();
                                match field_name.as_str() {
                                    "name" => data.name = value_str,
                                    "customer_id" => data.customer_id = value_str,
                                    "phone" => data.phone = value_str,
                                    "age" => data.age = value_str.parse().unwrap_or(0),
                                    "address" => data.address = value_str,
                                    "occupation" => data.occupation = value_str,
                                    "add_date" => data.add_date = value_str.replace("/", "-"),
                                    "userornot" => data.userornot = value_str,
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                //資料邏輯判斷
                if data.name.len()>12 {
                    return error_msg("客戶姓名應該小於13").await;
                }
                if data.name.len()==0 {
                    return error_msg("客戶姓名不能為空").await;
                }
                if data.customer_id.len()!=10 {
                    return error_msg("身分證字號應該為十碼").await;
                }
                if data.phone.len()!=10 {
                    return error_msg("電話號碼應該為十碼").await;
                }
                if data.age <0 {
                    return error_msg("年齡不應該是負數").await;
                }
                if data.age == 0 {
                    return error_msg("年齡不能為空").await;
                }
                if data.address.len() == 0 {
                    return error_msg("地址不能為空").await;
                }
                if data.address.len() > 30 {
                    return error_msg("地址不能超過30字").await;
                }
                if data.occupation.len() > 12 {
                    return error_msg("職業最多12個字").await;
                }
                if !date_check(&data.address) {
                    return error_msg("日期格式錯誤").await;
                } 

                let mut conn = match db.get_conn() {
                    Ok(conn) => conn,
                    Err(_) => return error_handler().await,
                };
                let login_params = params! {
                    "id_number" => &data.customer_id,
                    "phone_number" => &data.phone,
                    "is_admin" => 0,
                };

                let login_query = "INSERT INTO login_data (id_number, phone_number, is_admin) VALUES (:id_number, :phone_number, :is_admin)";
                match conn.exec_drop(login_query, login_params) {
                    Ok(_) => {
                        let customer_params = params! {
                            "customer_name"=>&data.name,
                            "id_number"=>&data.customer_id,
                            "phone_number"=>&data.phone,
                            "address"=>&data.address,
                            "age"=>&data.age,
                            "occupation"=>&data.occupation,
                            "registration_date"=>&data.add_date,
                            "photo"=>photo,
                            "status"=>&data.userornot
                        };
                        let customer_query = "INSERT INTO customer_data (
                            customer_name, id_number, phone_number, address, age, 
                            occupation, registration_date, photo, status
                        ) VALUES (
                            :customer_name, :id_number, :phone_number, :address, :age, 
                            :occupation, :registration_date, :photo, :status
                        )";
                        match conn.exec_drop(customer_query, customer_params) {
                            Ok(_) => {
                                let response_body = serde_json::json!({
                                    "create_flag": "true",
                                });
                                return HttpResponse::Ok()
                                    .content_type("application/json")
                                    .json(response_body);
                            }
                            Err(_e) => {
                                println!("建立客戶資料失敗，{}",_e);
                                let params = params! {
                                    "id_number" => &data.customer_id,
                                };
                                let delete_query =
                                    "DELETE FROM login_data WHERE id_number = :id_number";
                                match conn.exec_drop(delete_query, params) {
                                    Ok(_) => {
                                        let response_body = serde_json::json!({
                                            "create_flag": "false",
                                        });
                                        return HttpResponse::Ok()
                                            .content_type("application/json")
                                            .json(response_body);
                                    }
                                    Err(_e) => {
                                        eprintln!("Failed to delete customer data: {}", _e);
                                    }
                                }
                                return error_handler().await;
                            }
                        }
                    }
                    Err(_e) => {
                        println!("{}", _e);
                        let response_body = serde_json::json!({
                            "create_flag": "duplicate_id",
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