use crate::http_status::{error_401, error_handler, permission_denied};
use actix_session::Session;
use actix_web::{HttpRequest, HttpResponse};
pub fn verification_session(session: &Session) -> bool {
    match session.get::<String>("id") {
        Ok(Some(_id)) => {
            return true;
        }
        Ok(None) => {
            return false;
        }
        Err(_e) => {
            return false;
        }
    }
}
pub fn verification_admin(session: &Session) -> bool {
    match session.get::<bool>("is_admin") {
        Ok(Some(is_admin)) => {
            return is_admin;
        }
        Ok(None) => {
            return false;
        }
        Err(_e) => {
            return false;
        }
    }
}

pub async fn login(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("html/loginPage.html"))
}

pub async fn mainpage(session: Session) -> HttpResponse {
    if verification_session(&session) {
        session
            .insert("location", "mainpage")
            .expect("location set failed");
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(include_str!("html/mainpage.html"));
    }
    return error_401().await;
}

//客戶訂貨紀錄相關跳轉
pub async fn order_interface(session: Session) -> HttpResponse {
    if verification_session(&session) {
        session
            .insert("location", "order_interface")
            .expect("location set failed");
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(include_str!("html/order_interface/order_interface.html"));
    }
    return error_401().await;
}
pub async fn add_order(session: Session, _req: HttpRequest) -> HttpResponse {
    // println!("add_order HttpRequest:\n {:?}",_req);
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "order_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/order_interface/add_order.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn search_order(session: Session, _req: HttpRequest) -> HttpResponse {
    if verification_session(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "order_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/order_interface/search_order.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return error_401().await;
}
pub async fn watch_personalorder(session: Session) -> HttpResponse {
    if verification_session(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "order_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/order_interface/watch_personalorder.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return error_401().await;
}
pub async fn watch_allamountorder(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "order_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/order_interface/watch_allamountorder.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn watch_allorder(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "order_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/order_interface/watch_allorder.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
//客戶基本資料
pub async fn client_interface(session: Session) -> HttpResponse {
    if verification_session(&session) {
        session
            .insert("location", "client_interface")
            .expect("location set failed");
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(include_str!("html/client_interface/client_interface.html"));
    }
    return error_401().await;
}
pub async fn add_client(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/add_client.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn delete_client(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/delete_client.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn modify_client(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/modify_client.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn search_client(session: Session) -> HttpResponse {
    if verification_session(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/search_client.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn month_allclient(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/month_allclient.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn week_allclient(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/week_allclient.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn month_client(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/month_client.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn week_client(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "client_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/client_interface/week_client.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
//公司進貨資料
pub async fn purchase_interface(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        session
            .insert("location", "purchase_interface")
            .expect("location set failed");
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(include_str!(
                "html/purchase_interface/purchase_interface.html"
            ));
    }
    return error_401().await;
}
pub async fn add_purchase(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "purchase_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/purchase_interface/add_purchase.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn client_purchase(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "purchase_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!("html/purchase_interface/client_purchase.html"));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_day_product_data_purchase(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "purchase_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/purchase_interface/supplier_day_product_data_purchase.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_day_total_money_purchase(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "purchase_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/purchase_interface/supplier_day_total_money_purchase.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_week_total_money_purchase(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "purchase_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/purchase_interface/supplier_week_total_money_purchase.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_week_product_data_purchase(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "purchase_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/purchase_interface/supplier_week_product_data_purchase.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
//公司應收帳款
pub async fn receivables_interface(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        session
            .insert("location", "receivables_interface")
            .expect("location set failed");
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(include_str!(
                "html/receivables_interface/receivables_interface.html"
            ));
    }
    return error_401().await;
}
pub async fn add_receivables(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "receivables_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/receivables_interface/add_receivables.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn delete_receivables(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "receivables_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/receivables_interface/delete_receivables.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn modify_receivables(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "receivables_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/receivables_interface/modify_receivables.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn search_receivables(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "receivables_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/receivables_interface/search_receivables.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
//整合資料
pub async fn all_interface(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        session
            .insert("location", "all_interface")
            .expect("location set failed");
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(include_str!("html/all_interface/all_interface.html"));
    }
    return error_401().await;
}
pub async fn client_week_money_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/client_week_money_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_day_money_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/supplier_day_money_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn client_month_money_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/client_month_money_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_week_money_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/supplier_week_money_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_day_product_data_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/supplier_day_product_data_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn supplier_week_product_data_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/supplier_week_product_data_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn week_client_money_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/week_client_money_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
pub async fn week_all_client_money_all(session: Session) -> HttpResponse {
    if verification_admin(&session) {
        match session.get::<String>("location") {
            Ok(Some(location)) => {
                if location == "all_interface" {
                    return HttpResponse::Ok()
                        .content_type("text/html")
                        .body(include_str!(
                            "html/all_interface/week_all_client_money_all.html"
                        ));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().body(include_str!("html/404.html"));
            }
            Err(_e) => {
                return error_handler().await;
            }
        }
    }
    return permission_denied().await;
}
