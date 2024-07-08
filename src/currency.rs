use {chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Utc}, reqwest::get, roxmltree::{Document, Node}};

static mut LAST_CURRENCY: String = String::new();
static mut LAST_UPDATE_DATETIME: NaiveDateTime = NaiveDateTime::new(NaiveDate::MIN, NaiveTime::MIN);
static UPDATE_COOLDOWN: TimeDelta = TimeDelta::hours(1);

async fn get_current_currency() -> String {
    let response_body = match get("https://www.cbr.ru/scripts/XML_daily.asp").await.unwrap().text().await {
        Ok(str) => str,
        Err(error) => panic!("{error:?}")
    };
    
    let doc: Document = Document::parse(response_body.as_str()).unwrap();
    let children_text = match match match doc
    .descendants().find(|parent: &Node| parent.attribute("ID") == Some("R01235")) {
        Some(parent) => parent,
        None => panic!("Parent not found!")
    }.children().find(|children| children.has_tag_name("Value")) {
        Some(children) => children,
        None => panic!("Children not found!")
    }.text() {
        Some(children_text) => children_text,
        None => panic!("Children Text not found!")
    };

    String::from(children_text)
}

pub async fn get_value() -> String {
    let now: NaiveDateTime = Utc::now().naive_utc();
    unsafe {
        if LAST_UPDATE_DATETIME + UPDATE_COOLDOWN < now {
            LAST_CURRENCY = get_current_currency().await;
            LAST_UPDATE_DATETIME = now;
        }
        LAST_CURRENCY.clone()
    }
}