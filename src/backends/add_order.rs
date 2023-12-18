use crate::http_status::{error_401, error_handler, permission_denied};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Data {
    customer_id: String,
    order_date: String,
    expected_delivery_date: String,
    actual_delivery_date: String,
    product_name: String,
    unit: String,
    quantity: i32,
    unit_price: f32,
    total: f32,
    supplier_name: String,
    supplier_id: String,
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
                let product_name = data.product_name.clone();
                let unit = data.unit.clone();
                let quantity = data.quantity;
                let unit_price = data.unit_price;
                let order_amount = data.total;
                let supplier_name = data.supplier_name.clone();
                let supplier_id = data.supplier_id.clone();

                // Creating a connection from the pool
                let mut conn = match db.get_conn() {
                    Ok(conn) => conn,
                    Err(_) => return error_handler().await,
                };

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
