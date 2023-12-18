use crate::http_status::{error_401, error_handler};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use chrono::NaiveDate;
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct Data {
    supplier: String,
}
#[derive(Debug, Serialize)]
pub struct SendData {
    supplier_name: String,
    supplier_id: String,
    supplier_contact_person: String,
    product_name: String,
    unit: String,
    unit_price: f32,
    subtotal: f32,
    stock_location: String,
    specification: String,
    purchase_date: NaiveDate,
    quantity: i32,
}
pub async fn client_purchase_data(
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
                        "SELECT * FROM PurchaseData WHERE SupplierName = :supplier ",
                        params! {
                            "supplier" => &data.supplier,
                        },
                        |row: Row| SendData {
                            supplier_name: row.get("SupplierName").unwrap_or_default(),
                            supplier_id: row.get("SupplierID").unwrap_or_default(),
                            supplier_contact_person: row
                                .get("SupplierContactPerson")
                                .unwrap_or_default(),
                            product_name: row.get("ProductName").unwrap_or_default(),
                            unit: row.get("Unit").unwrap_or_default(),
                            unit_price: row.get("UnitPrice").unwrap(),
                            subtotal: row.get("Subtotal").unwrap(),
                            stock_location: row.get("StockLocation").unwrap_or_default(),
                            specification: row.get("Specification").unwrap_or_default(),
                            purchase_date: row.get("PurchaseDate").unwrap_or_default(),
                            quantity: row.get("Quantity").unwrap_or_default(),
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
