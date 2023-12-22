use crate::http_status::{error_401, error_handler, permission_denied};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use crate::backends::library::{error_msg,date_check,compare_dates};
#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
    order_date: String,
    expected_delivery_date: String,
    actual_delivery_date: String,
    // product_name: String,
    // unit: String,
    quantity: i32,
    // unit_price: f32,
    // total: f32,
    // supplier_name: String,
    supplier_id: String,
}
struct SupplierData {
    product_name: String,
    unit: String,
    unit_price: f32,
    supplier_name: String,
}

pub async fn create_order(
    data: web::Json<Data>,
    session: Session,
    db: web::Data<Pool>,
) -> HttpResponse {
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            if is_admin {
                // Extracting values from the received JSON data
                let id_number = data.customer_id.clone(); // Assuming customerID corresponds to id_number
                let order_date = data.order_date.clone().replace("/", "-");
                let expected_delivery_date = data.expected_delivery_date.clone().replace("/", "-");
                let actual_delivery_date = data.actual_delivery_date.clone().replace("/", "-");
                // let product_name = data.product_name.clone();
                // let unit = data.unit.clone();
                let quantity = data.quantity;
                // let unit_price = data.unit_price;
                // let order_amount = data.total;
                // let supplier_name = data.supplier_name.clone();
                let supplier_id = &data.supplier_id;
                // Creating a connection from the pool
                let mut conn = match db.get_conn() {
                    Ok(conn) => conn,
                    Err(_) => return error_handler().await,
                };
                let supplier_data: Option<SupplierData> = match conn.exec_first(
                    "SELECT SupplierName, Unit, UnitPrice, ProductName FROM PurchaseData WHERE SupplierID = :supplier_id ",
                    params! {
                        "supplier_id" => &data.supplier_id,
                    }
                ) {
                    Ok(data) => data.map(|(supplier_name, unit, unit_price, product_name)| {
                        SupplierData {
                            supplier_name,
                            unit,
                            unit_price,
                            product_name,
                        }
                    }),
                    Err(e) => {
                        eprintln!("Database query error: {:?}", e);
                        return error_handler().await; 
                    }
                };
                let (product_name,unit_price,supplier_name,unit) = match supplier_data {
                    Some(data)=>{
                        (data.product_name,data.unit_price,data.supplier_name,data.unit)
                    }
                    None=>{
                        return error_msg("查無此供應商編號").await;
                    }
                };
                let order_amount:f32 = quantity as f32 * unit_price;
                if id_number.len() !=10 {
                    return error_msg("身分證字號應該為十碼").await;
                }
                if !date_check(&order_date) {
                    return error_msg("訂單日期格式錯誤").await;
                } 
                if !date_check(&expected_delivery_date) {
                    return error_msg("預計遞交日期格式錯誤").await;
                } 
                if !date_check(&actual_delivery_date) {
                    return error_msg("實際遞交日期格式錯誤").await;
                } 
                if !compare_dates(&order_date, &expected_delivery_date){
                    return error_msg("預計遞交日期不會比訂單日期早").await;
                }
                if !compare_dates(&order_date, &actual_delivery_date){
                    return error_msg("實際遞交日期不會比訂單日期早").await;
                }
                

                // Executing the insertion query
                let query = format!(
                    "INSERT INTO order_records (id_number, order_date, expected_delivery_date, actual_delivery_date, product_name, unit, quantity, unit_price, order_amount, supplier_name, supplier_id) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {}, {}, {}, '{}', '{}')",
                    id_number, order_date, expected_delivery_date, actual_delivery_date, product_name, unit, quantity, unit_price, order_amount, supplier_name, supplier_id
                );

                match conn.query_drop(&query) {
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
