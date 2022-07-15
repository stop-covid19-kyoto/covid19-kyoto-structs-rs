use chrono::{
    DateTime,
	Local,
	TimeZone,
	Utc
};
use crate::{
	structs::{
		last_update::LastUpdate,
		status::{
			Attributes as StatusAttributes,
			DetailedStatus,
			Status
		},
		summary::Summary,
		summary::SummaryContent
	},
	utils::formats::FORMAT
};

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

/// DetailedStatus構造体のシリアライズのテストを行います。
#[test]
fn serialize_detailed_status() {
	assert!(serde_json::to_string(&test_data_detailed_status()).is_ok());
}

/// DetailedStatus構造体のデシリアライズのテストを行います。
#[test]
fn deserialize_detailed_status() {
	let serialized = r#"{"attr":"leave","value":2048}"#;
	let deserialized = serde_json::from_str::<DetailedStatus>(&serialized);
    assert!(deserialized.is_ok());
}

/// Status構造体のシリアライズのテストを行います。
#[test]
fn serialize_status() {
	assert!(serde_json::to_string(&test_data_status()).is_ok());
}

/// Status構造体のデシリアライズのテストを行います。
#[test]
fn deserialize_status() {
	let serialized = r#"{"attr":"patients","value":3072,"children":[{"attr":"leave","value":2048}]}"#;
	let deserialized = serde_json::from_str::<Status>(&serialized);
    assert!(deserialized.is_ok());
}

/// LastUpdate構造体のテスト用のデータを生成します。
fn test_data_last_update() -> LastUpdate<Local> {
    return LastUpdate {
        datetime: dummy_localdate()
    };
}

/// SummaryContent構造体のテスト用のデータを生成します。
fn test_data_summary_content() -> SummaryContent {
	return SummaryContent {
		date: "2020-03-25T09:25:00.000Z".parse::<DateTime<Utc>>().unwrap(),
		sum: 10
	};
}

/// Summary構造体のテスト用のデータを生成します。
fn test_data_summary() -> Summary {
	return Summary {
		data: vec![test_data_summary_content()],
		last_update: dummy_localdate()
	};
}

/// DetailedStatus構造体のテスト用のデータを生成します。
fn test_data_detailed_status() -> DetailedStatus {
	return DetailedStatus {
		attr: StatusAttributes::Leave,
		value: 2048
	}
}

/// Status構造体のテスト用のデータを生成します。
fn test_data_status() -> Status {
	return Status {
		attr: StatusAttributes::Patients,
		value: 3072,
		children: vec![test_data_detailed_status()]
	}
}

/// 2020年3月25日 21時40分をDateTime<Local>型で表現し、これをダミーのデータとして扱います。
/// 
/// このダミーデータで扱う日付・時刻は、対策サイトが産声を上げた瞬間を指しています。
fn dummy_localdate() -> DateTime<Local> {
	return Local.datetime_from_str("2020/03/25 21:40", FORMAT).unwrap();
}
