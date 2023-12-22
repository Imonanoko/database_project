use actix_web::HttpResponse;
use chrono::NaiveDate;
pub async fn error_msg(msg:&str)->HttpResponse{
    let response_body = serde_json::json!({
        "create_flag":"error message",
        "error_msg":msg,
    });
    return HttpResponse::Ok()
    .content_type("application/json")
    .json(response_body);
}
pub fn date_check(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();

    // 檢查是否正確分割為年、月、日
    if parts.len() != 3 {
        return false;
    }

    // 解析年、月、日
    let year = parts[0].parse::<i32>();
    let month = parts[1].parse::<u32>();
    let day = parts[2].parse::<u32>();

    // 檢查年、月、日是否有效
    match (year, month, day) {
        (Ok(y), Ok(m), Ok(d)) => {
            // 使用 chrono 包來檢查日期的有效性
            NaiveDate::from_ymd_opt(y, m, d).is_some()
        }
        _ => false,
    }
}