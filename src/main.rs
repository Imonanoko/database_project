use actix_files as fs;
use actix_session::CookieSession;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use mysql::*;
use rand::RngCore;
use std::path::PathBuf;
mod backends {
    pub mod add_client_data;
    pub mod add_order;
    pub mod add_purchase_data;
    pub mod add_receivables_data;
    pub mod client_data;
    pub mod client_month_money_all_data;
    pub mod client_purchase_data;
    pub mod client_week_money_all_data;
    pub mod delete_client_data;
    pub mod delete_receivables_data;
    pub mod login;
    pub mod modify_client_modify_data;
    pub mod modify_client_search_data;
    pub mod modify_receivables_data;
    pub mod modify_receivables_search_data;
    pub mod month_allclient_data;
    pub mod month_client_data;
    pub mod search_allamountorder_data;
    pub mod search_allorder_data;
    pub mod search_client_data;
    pub mod search_delete_client_data;
    pub mod search_order_data;
    pub mod search_personalorder_data;
    pub mod search_receivables_data;
    pub mod supplier_day_money_all_data;
    pub mod supplier_day_product_data_all_data;
    pub mod supplier_day_product_purchase_data;
    pub mod supplier_day_totalmoney_data;
    pub mod supplier_week_product_data_all_data;
    pub mod supplier_week_product_purchase_data;
    pub mod supplier_week_totalmoney_data;
    pub mod week_all_client_money_all_data;
    pub mod week_allclient_data;
    pub mod week_client_data;
    pub mod week_client_money_all_data;
    pub mod library;
}
mod http_status;
mod jump_page;
use backends::{
    add_client_data, add_order, add_purchase_data, add_receivables_data, client_data,
    client_month_money_all_data, client_purchase_data, client_week_money_all_data,
    delete_client_data, delete_receivables_data, login, modify_client_modify_data,
    modify_client_search_data, modify_receivables_data, modify_receivables_search_data,
    month_allclient_data, month_client_data, search_allamountorder_data, search_allorder_data,
    search_client_data, search_delete_client_data, search_order_data, search_personalorder_data,
    search_receivables_data, supplier_day_money_all_data, supplier_day_product_data_all_data,
    supplier_day_product_purchase_data, supplier_day_totalmoney_data,
    supplier_week_product_data_all_data, supplier_week_product_purchase_data,
    supplier_week_totalmoney_data, week_all_client_money_all_data, week_allclient_data,
    week_client_data, week_client_money_all_data,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let static_files_css = PathBuf::from("./src/css");
    let static_files_images = PathBuf::from("./src/images");
    let database_url = "mysql://root:Imonanoko@localhost:3306/company_data";
    let pool = Pool::new(database_url).unwrap();
    //生成隨機的key加密session
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(CookieSession::signed(&key).secure(false).http_only(true).same_site(actix_web::cookie::SameSite::Strict))
            .service(fs::Files::new("/images", &static_files_images).show_files_listing())
            .service(fs::Files::new("/css", &static_files_css).show_files_listing()) // 添加静态文件服务
            .service(web::resource("/").to(jump_page::login)) //跳轉登入介面
            .service(
                web::resource("/login_verification")
                    .route(web::post().to(login::login_verification)),
            ) //處理登入後端
            .service(web::resource("/mainpage").to(jump_page::mainpage)) //都入後進入主頁面
            //客戶訂貨紀錄
            .service(web::resource("/order_interface").to(jump_page::order_interface))
            .service(web::resource("/order_interface/add_order").to(jump_page::add_order)) //客戶訂貨紀錄的新增訂單
            .service(
                web::resource("/order_interface/create_order")
                    .route(web::post().to(add_order::create_order)),
            ) //客戶訂貨紀錄的新增訂單的處理後端
            .service(web::resource("/order_interface/search_order").to(jump_page::search_order)) //客戶訂貨紀錄的查詢訂單
            .service(
                web::resource("/order_interface/search_order_data")
                    .route(web::post().to(search_order_data::search_order_data)),
            ) //客戶訂貨紀錄的查詢訂單的處理後端
            .service(
                web::resource("/order_interface/watch_personalorder")
                    .to(jump_page::watch_personalorder),
            ) //客戶訂貨紀錄的查詢某人的訂單
            .service(
                web::resource("/order_interface/search_personalorder_data")
                    .route(web::post().to(search_personalorder_data::search_personalorder_data)),
            ) //客戶訂貨紀錄的查詢某人的訂單的處理後端
            .service(
                web::resource("/order_interface/watch_allamountorder")
                    .to(jump_page::watch_allamountorder),
            ) //客戶訂貨紀錄的某一星期全體客戶每一人之訂貨總金額
            .service(
                web::resource("/order_interface/search_allamountorder_data")
                    .route(web::post().to(search_allamountorder_data::search_allamountorder_data)),
            ) //客戶訂貨紀錄的某一星期全體客戶每一人之訂貨總金額的處理後端
            .service(web::resource("/order_interface/watch_allorder").to(jump_page::watch_allorder)) //客戶訂貨紀錄的某一星期全體客戶每一人之訂貨總金額
            .service(
                web::resource("/order_interface/search_allorder_data")
                    .route(web::post().to(search_allorder_data::search_allorder_data)),
            ) //客戶訂貨紀錄的某一星期全體客戶每一人之訂貨總金額的處理後端
            //客戶基本資料
            .service(web::resource("/client_interface").to(jump_page::client_interface))
            .service(web::resource("/client_interface/client_data").to(client_data::client_data))
            .service(web::resource("/client_interface/add_client").to(jump_page::add_client))
            .service(
                web::resource("/client_interface/add_client_data")
                    .route(web::post().to(add_client_data::add_client_data)),
            )
            .service(web::resource("/client_interface/delete_client").to(jump_page::delete_client))
            .service(
                web::resource("/client_interface/delete_client_data")
                    .route(web::post().to(delete_client_data::delete_client_data)),
            )
            .service(
                web::resource("/client_interface/search_delete_client_data")
                    .route(web::post().to(search_delete_client_data::search_delete_client_data)),
            )
            .service(web::resource("/client_interface/modify_client").to(jump_page::modify_client))
            .service(
                web::resource("/client_interface/modify_client_search_data")
                    .route(web::post().to(modify_client_search_data::modify_client_search_data)),
            )
            .service(
                web::resource("/client_interface/modify_client_modify_data")
                    .route(web::post().to(modify_client_modify_data::modify_client_modify_data)),
            )
            .service(web::resource("/client_interface/search_client").to(jump_page::search_client))
            .service(
                web::resource("/client_interface/search_client_data")
                    .route(web::post().to(search_client_data::search_client_data)),
            )
            .service(
                web::resource("/client_interface/month_allclient").to(jump_page::month_allclient),
            )
            .service(
                web::resource("/client_interface/month_allclient_data")
                    .route(web::post().to(month_allclient_data::month_allclient_data)),
            )
            .service(
                web::resource("/client_interface/week_allclient").to(jump_page::week_allclient),
            )
            .service(
                web::resource("/client_interface/week_allclient_data")
                    .route(web::post().to(week_allclient_data::week_allclient_data)),
            )
            .service(web::resource("/client_interface/month_client").to(jump_page::month_client))
            .service(
                web::resource("/client_interface/month_client_data")
                    .route(web::post().to(month_client_data::month_client_data)),
            )
            .service(web::resource("/client_interface/week_client").to(jump_page::week_client))
            .service(
                web::resource("/client_interface/week_client_data")
                    .route(web::post().to(week_client_data::week_client_data)),
            )
            //公司進貨資料
            .service(web::resource("/purchase_interface").to(jump_page::purchase_interface))
            .service(web::resource("/purchase_interface/add_purchase").to(jump_page::add_purchase))
            .service(
                web::resource("/purchase_interface/add_purchase_data")
                    .route(web::post().to(add_purchase_data::add_purchase_data)),
            )
            .service(
                web::resource("/purchase_interface/client_purchase").to(jump_page::client_purchase),
            )
            .service(
                web::resource("/purchase_interface/client_purchase_data")
                    .route(web::post().to(client_purchase_data::client_purchase_data)),
            )
            .service(
                web::resource("/purchase_interface/supplier_day_product_data_purchase")
                    .to(jump_page::supplier_day_product_data_purchase),
            )
            .service(
                web::resource("/purchase_interface/supplier_day_product_purchase_data")
                    .route(web::post().to(
                        supplier_day_product_purchase_data::supplier_day_product_purchase_data,
                    )),
            )
            .service(
                web::resource("/purchase_interface/supplier_day_total_money_purchase")
                    .to(jump_page::supplier_day_total_money_purchase),
            )
            .service(
                web::resource("/purchase_interface/supplier_day_totalmoney_data").route(
                    web::post().to(supplier_day_totalmoney_data::supplier_day_totalmoney_data),
                ),
            )
            .service(
                web::resource("/purchase_interface/supplier_week_total_money_purchase")
                    .to(jump_page::supplier_week_total_money_purchase),
            )
            .service(
                web::resource("/purchase_interface/supplier_week_totalmoney_data").route(
                    web::post().to(supplier_week_totalmoney_data::supplier_week_totalmoney_data),
                ),
            )
            .service(
                web::resource("/purchase_interface/supplier_week_product_data_purchase")
                    .to(jump_page::supplier_week_product_data_purchase),
            )
            .service(
                web::resource("/purchase_interface/supplier_week_product_purchase_data").route(
                    web::post().to(
                        supplier_week_product_purchase_data::supplier_week_product_purchase_data,
                    ),
                ),
            )
            //公司應收帳款
            .service(web::resource("/receivables_interface").to(jump_page::receivables_interface))
            .service(
                web::resource("/receivables_interface/add_receivables")
                    .to(jump_page::add_receivables),
            )
            .service(
                web::resource("/receivables_interface/add_receivables_data")
                    .route(web::post().to(add_receivables_data::add_receivables_data)),
            )
            .service(
                web::resource("/receivables_interface/delete_receivables")
                    .to(jump_page::delete_receivables),
            )
            .service(
                web::resource("/receivables_interface/delete_receivables_data")
                    .route(web::post().to(delete_receivables_data::delete_receivables_data)),
            )
            .service(
                web::resource("/receivables_interface/modify_receivables")
                    .to(jump_page::modify_receivables),
            )
            .service(
                web::resource("/receivables_interface/modify_receivables_data")
                    .route(web::post().to(modify_receivables_data::modify_receivables_data)),
            )
            .service(
                web::resource("/receivables_interface/modify_receivables_search_data").route(
                    web::post().to(modify_receivables_search_data::modify_receivables_search_data),
                ),
            )
            .service(
                web::resource("/receivables_interface/search_receivables")
                    .to(jump_page::search_receivables),
            )
            .service(
                web::resource("/receivables_interface/search_receivables_data")
                    .route(web::post().to(search_receivables_data::search_receivables_data)),
            )
            //整合資料
            .service(web::resource("/all_interface").to(jump_page::all_interface))
            .service(
                web::resource("/all_interface/client_week_money_all")
                    .to(jump_page::client_week_money_all),
            )
            .service(
                web::resource("/all_interface/client_week_money_all_data")
                    .route(web::post().to(client_week_money_all_data::client_week_money_all_data)),
            )
            .service(
                web::resource("/all_interface/supplier_day_money_all")
                    .to(jump_page::supplier_day_money_all),
            )
            .service(
                web::resource("/all_interface/supplier_day_money_all_data").route(
                    web::post().to(supplier_day_money_all_data::supplier_day_money_all_data),
                ),
            )
            .service(
                web::resource("/all_interface/client_month_money_all")
                    .to(jump_page::client_month_money_all),
            )
            .service(
                web::resource("/all_interface/client_month_money_all_data").route(
                    web::post().to(client_month_money_all_data::client_month_money_all_data),
                ),
            )
            .service(
                web::resource("/all_interface/supplier_week_money_all")
                    .to(jump_page::supplier_week_money_all),
            )
            .service(
                web::resource("/all_interface/supplier_day_product_data_all")
                    .to(jump_page::supplier_day_product_data_all),
            )
            .service(
                web::resource("/all_interface/supplier_day_product_data_all_data")
                    .route(web::post().to(
                        supplier_day_product_data_all_data::supplier_day_product_data_all_data,
                    )),
            )
            .service(
                web::resource("/all_interface/supplier_week_product_data_all")
                    .to(jump_page::supplier_week_product_data_all),
            )
            .service(
                web::resource("/all_interface/supplier_week_product_data_all_data").route(
                    web::post().to(
                        supplier_week_product_data_all_data::supplier_week_product_data_all_data,
                    ),
                ),
            )
            .service(
                web::resource("/all_interface/week_client_money_all")
                    .to(jump_page::week_client_money_all),
            )
            .service(
                web::resource("/all_interface/week_client_money_all_data")
                    .route(web::post().to(week_client_money_all_data::week_client_money_all_data)),
            )
            .service(
                web::resource("/all_interface/week_all_client_money_all")
                    .to(jump_page::week_all_client_money_all),
            )
            .service(
                web::resource("/all_interface/week_all_client_money_all_data").route(
                    web::post().to(week_all_client_money_all_data::week_all_client_money_all_data),
                ),
            )
            .default_service(web::route().to(http_status::error_404)) //其他跳轉到404
    })
    .bind(("140.128.101.24", 8888))?
    .run()
    .await
}
