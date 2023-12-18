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
    id: String,
}
#[derive(Debug, Serialize)]
pub struct SendData {
    client_name: String,
    client_phone: String,
    client_age: i32,
    client_address: String,
    client_occupation: String,
    client_adddate: NaiveDate,
    client_useorno: String,
    receivables_date: NaiveDate,
    receivables_money: f32,
    receivables_hastenmoney: f32,
}
pub async fn search_receivables_data(
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
                            customer_data.customer_name AS client_name,
                            customer_data.phone_number AS client_phone,
                            customer_data.age AS client_age,
                            customer_data.address AS client_address,
                            customer_data.occupation AS client_occupation,
                            customer_data.registration_date AS client_adddate,
                            customer_data.status AS client_useorno,
                            AccountsReceivable.DueDate AS receivables_date,
                            AccountsReceivable.ReceivableAmount AS receivables_money,
                            AccountsReceivable.PendingCollectionAmount AS receivables_hastenmoney
                            FROM
                            customer_data
                            JOIN
                            AccountsReceivable ON customer_data.id_number = AccountsReceivable.IDNumber
                            WHERE
                            customer_data.id_number = :id;",
                        params! {
                            "id" => &data.id,
                        },
                        |row: mysql::Row| {
                            let client_name: String = row.get("client_name").unwrap();
                            let client_phone: String = row.get("client_phone").unwrap();
                            let client_age: i32 = row.get("client_age").unwrap();
                            let client_address: String = row.get("client_address").unwrap();
                            let client_occupation: String = row.get("client_occupation").unwrap();
                            let client_adddate: NaiveDate = row.get("client_adddate").unwrap();
                            let receivables_money: f32 = row.get("receivables_money").unwrap();
                            let receivables_hastenmoney: f32 = row.get("receivables_hastenmoney").unwrap();
                            let client_useorno: String = row.get("client_useorno").unwrap();
                            let receivables_date: NaiveDate = row.get("receivables_date").unwrap();
                            SendData {
                                client_name,
                                client_phone,
                                client_age,
                                client_address,
                                client_occupation,
                                client_adddate,
                                client_useorno,
                                receivables_date,
                                receivables_money,
                                receivables_hastenmoney,
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
