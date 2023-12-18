use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Data {
    supplier: String,
    product: String,
    start_date: String,
    end_date: String,
}

#[derive(Debug, Serialize)]
pub struct SendData {
    supplier_name: String,
    week_product_name: String,
    week_total_amount: i32,
    week_purchase_money: f32,
}
pub async fn supplier_week_product_data_all_data(
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
                        "SELECT SupplierName, ProductName, Quantity ,Subtotal \
                        FROM PurchaseData \
                        WHERE SupplierName = :supplier AND ProductName = :product AND PurchaseDate BETWEEN :start_date AND :end_date",
                        params! {
                            "supplier" => &data.supplier,
                            "product" => &data.product,
                            "start_date"=>&data.start_date.split('T')
                            .next()
                            .unwrap_or_default()
                            .to_string()
                            .replace("/", "-"),
                            "end_date"=> &data.end_date.split('T')
                            .next()
                            .unwrap_or_default()
                            .to_string()
                            .replace("/", "-"),
                        },
                        |row: mysql::Row| {
                            let supplier_name: String = row.get("SupplierName").unwrap();
                            let week_product_name: String = row.get("ProductName").unwrap();
                            let week_total_amount: i32 = row.get("Quantity").unwrap();
                            let week_purchase_money:f32 = row.get("Subtotal").unwrap(); 
                            SendData {
                                supplier_name,
                                week_product_name,
                                week_total_amount,
                                week_purchase_money,
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
