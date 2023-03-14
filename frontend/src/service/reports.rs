use super::fetch::Fetch;
use crate::entities::interfaces::InputReport;
use crate::entities::interfaces::Status;
use crate::API_URL;

pub async fn create_report(report: &InputReport) -> Result<(), Status> {
    let url = format!("{}/reports", API_URL);
    let string_report = format!(
        "{{\"article_id\": {}, \"report_cat\":\"{}\", \"description\":\"{}\"}}",
        report.article_id, report.report_cat, report.description
    );
    let jsreport = wasm_bindgen::JsValue::from_str(&string_report);
    let json = Fetch::post(url, &jsreport).await;
    match json {
        Ok(_json) => Ok(()),
        Err(_err) => Err(Status::Error),
    }
}
