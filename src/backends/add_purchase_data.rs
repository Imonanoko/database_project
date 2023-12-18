use crate::http_status::{error_401, error_handler, permission_denied};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Data {
    supplier_id: String,
    supplier_person: String,
    supplier_name: String,
    product_name: String,
    place: String,
    specification: String,
    unit: String,
    quantity: i32,
    unit_price: f32,
    sub_total: f32,
    purchase_date: String,
}

pub async fn add_purchase_data(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                // Extracting values from the received JSON data
                let supplier_id = &data.supplier_id; // Assuming customerID corresponds to id_number
                let supplier_person = &data.supplier_person;
                let supplier_name = &data.supplier_name;
                let product_name = &data.product_name;
                let place = &data.place;
                let specification = &data.specification;
                let unit = &data.unit;
                let quantity = &data.quantity;
                let unit_price = &data.unit_price;
                let sub_total = &data.sub_total;
                let purchase_date = &data.purchase_date.replace("/", "-");

                // Creating a connection from the pool
                let mut conn = match db.get_conn() {
                    Ok(conn) => conn,
                    Err(_) => return error_handler().await,
                };

                // Executing the insertion query
                let query = "INSERT INTO PurchaseData
                (SupplierName, SupplierID, SupplierContactPerson, ProductName, Unit, UnitPrice, Subtotal, StockLocation, Specification, PurchaseDate, Quantity)
                VALUES
                (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

                let params = (
                    &supplier_name,
                    &supplier_id,
                    &supplier_person,
                    &product_name,
                    &unit,
                    unit_price, // 示例价格
                    sub_total,  // 示例小计
                    &place,
                    &specification,
                    &purchase_date, // 示例日期
                    quantity,       // 示例数量
                );
                match conn.exec_drop(query, params) {
                    Ok(_) => {
                        // If the query was successful
                        let response_body = serde_json::json!({
                            "create_flag": "true",
                        });
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(response_body);
                    }
                    Err(_err) => {
                        println!("{}", _err);
                        let response_body = serde_json::json!({
                            "create_flag": "false",
                        });
                        return HttpResponse::Ok()
                            .content_type("application/json")
                            .json(response_body);
                    }
                }
            } else {
                return permission_denied().await;
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
