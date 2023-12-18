use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Data {
    supplier: String,
    startdate: String,
    enddate: String,
}

#[derive(Debug, Serialize)]
pub struct SendData {
    supplier_id: String,
    week_money: f32,
}
pub async fn supplier_week_totalmoney_data(
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
                        "SELECT SupplierID, Subtotal \
                        FROM PurchaseData \
                        WHERE PurchaseDate BETWEEN :start_date AND :end_date AND SupplierName= :supplier",
                        params! {
                            "start_date" => &data.startdate.split('T')
                            .next()
                            .unwrap_or_default()
                            .to_string()
                            .replace("/", "-"),
                            "end_date"=>&data.enddate.split('T')
                            .next()
                            .unwrap_or_default()
                            .to_string()
                            .replace("/", "-"),
                            "supplier"=>&data.supplier,
                        },
                        |row: mysql::Row| {
                            let supplier_id: String = row.get("SupplierID").unwrap();
                            let week_money:f32 = row.get("Subtotal").unwrap(); 
                            SendData {
                                supplier_id,
                                week_money,
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
