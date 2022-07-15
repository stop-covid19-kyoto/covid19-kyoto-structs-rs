use crate::{
    structs::{
        last_update::LastUpdate,
        news::{NewsItem, NewsItems},
        status::{Attributes as StatusAttributes, Status},
        summary::Summary,
        summary::SummaryContent,
    },
    utils::formats::DATETIME_FORMAT,
};
use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};

/// LastUpdate構造体のシリアライズのテストを行います。
#[test]
fn serialize_last_update() {
    assert!(serde_json::to_string(&test_data_last_update()).is_ok());
}

/// LastUpdate構造体のデシリアライズのテストを行います。
#[test]
fn deserialize_last_update() {
    let serialized = r#"{"last_update":"2020/03/25 21:40"}"#;
    let deserialized = serde_json::from_str::<LastUpdate<Local>>(&serialized);
    assert!(deserialized.is_ok());
}

#[test]
/// NewsItem構造体のシリアライズのテストを行います。
fn serialize_news_item() {
    println!("{}", serde_json::to_string(&test_data_news_item()).unwrap());
    assert!(serde_json::to_string(&test_data_news_item()).is_ok());
}

#[test]
/// NewsItem構造体のデシリアライズのテストを行います。
fn deserialize_news_item() {
    let serialized = r#"{"date":"2020/03/25","text":"京都府 新型コロナウイルス感染症 対策サイト","url":"https://kyoto.stopcovid19.jp/"}"#;
    let deserialized = serde_json::from_str::<NewsItem>(&serialized);
    assert!(deserialized.is_ok());
}

#[test]
/// NewsItems構造体のシリアライズのテストを行います。
fn serialize_news_items() {
    println!(
        "{}",
        serde_json::to_string(&test_data_news_items()).unwrap()
    );
    assert!(serde_json::to_string(&test_data_news_items()).is_ok());
}

#[test]
/// NewsItems構造体のデシリアライズのテストを行います。
fn deserialize_news_items() {
    let serialized = r#"{"news_items":[{"date":"2020/03/25","text":"京都府 新型コロナウイルス感染症 対策サイト","url":"https://kyoto.stopcovid19.jp/"}]}"#;
    let deserialized = serde_json::from_str::<NewsItems>(&serialized);
    assert!(deserialized.is_ok());
}

/// Status構造体のシリアライズのテストを行います。
#[test]
fn serialize_status() {
    assert!(serde_json::to_string(&test_data_status()).is_ok());
    assert!(serde_json::to_string(&test_data_status_with_children()).is_ok());
}

/// Status構造体のデシリアライズのテストを行います。
#[test]
fn deserialize_status() {
    let serialized =
        r#"{"attr":"patients","value":4096,"children":[{"attr":"accommodations","value":32}]}"#;
    let deserialized = serde_json::from_str::<Status>(&serialized);
    assert!(deserialized.is_ok());
}

/// Summary構造体のシリアライズのテストを行います。
#[test]
fn serialize_summary() {
    assert!(serde_json::to_string(&test_data_summary()).is_ok());
}

/// Summary構造体のデシリアライズのテストを行います。
#[test]
fn deserialize_summary() {
    let serialized = r#"{"data":[{"date":"2020-03-25T09:40:00.000Z", "sum": 10}], "last_update":"2020/03/25 21:25"}"#;
    let deserialized = serde_json::from_str::<Summary>(&serialized);
    assert!(deserialized.is_ok());
}

/// SummaryContent構造体のシリアライズのテストを行います。
#[test]
fn serialize_summary_content() {
    assert!(serde_json::to_string(&test_data_summary_content()).is_ok());
}

/// SummaryContent構造体のデシリアライズのテストを行います。
#[test]
fn deserialize_summary_content() {
    let serialized = r#"{"date":"2020-03-25T09:40:00.000Z", "sum": 10}"#;
    let deserialized = serde_json::from_str::<SummaryContent>(&serialized);
    assert!(deserialized.is_ok());
}

/// LastUpdate構造体のテスト用のデータを生成します。
fn test_data_last_update() -> LastUpdate<Local> {
    return LastUpdate {
        datetime: dummy_localdate(),
    };
}

/// NewsItem構造体のテスト用のデータを生成します。
fn test_data_news_item() -> NewsItem {
    return NewsItem {
        date: NaiveDate::from_ymd(2020, 3, 25),
        text: "京都府 新型コロナウイルス感染症 対策サイト".to_string(),
        url: "https://kyoto.stopcovid19.jp/".to_string(),
    };
}

/// NewsItems構造体のテスト用のデータを生成します。
fn test_data_news_items() -> NewsItems {
    return NewsItems {
        news_items: vec![test_data_news_item()],
    };
}

/// Status構造体のテスト用データを生成します。
fn test_data_status() -> Status {
    return Status {
        attr: StatusAttributes::Accommodations,
        value: 32,
        children: None,
        last_update: None,
    };
}

/// Status構造体のテスト用のデータを生成します。(子属性を含む)
fn test_data_status_with_children() -> Status {
    return Status {
        attr: StatusAttributes::Patients,
        value: 4096,
        children: Some(vec![test_data_status()]),
        last_update: Some(dummy_localdate()),
    };
}

/// SummaryContent構造体のテスト用のデータを生成します。
fn test_data_summary_content() -> SummaryContent {
    return SummaryContent {
        date: "2020-03-25T09:25:00.000Z".parse::<DateTime<Utc>>().unwrap(),
        sum: 10,
    };
}

/// Summary構造体のテスト用のデータを生成します。
fn test_data_summary() -> Summary {
    return Summary {
        data: vec![test_data_summary_content()],
        last_update: dummy_localdate(),
    };
}

/// 2020年3月25日 21時40分をDateTime<Local>型で表現し、これをダミーのデータとして扱います。
///
/// このダミーデータで扱う日付・時刻は、対策サイトが産声を上げた瞬間を指しています。
fn dummy_localdate() -> DateTime<Local> {
    return Local
        .datetime_from_str("2020/03/25 21:40", DATETIME_FORMAT)
        .unwrap();
}
