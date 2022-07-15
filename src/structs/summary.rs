use std::{
	fmt::{
		Error,
		Formatter
	}
};
use serde::{
	Deserialize,
	Deserializer,
	de::{
		Error as DeserializationError,
		MapAccess,
		Visitor
	},
	Serialize,
	Serializer,
	ser::{
		SerializeStruct
	}
};
use chrono::{
	DateTime,
	Local,
	TimeZone,
	Utc
};
use crate::{
	utils::formats::FORMAT
};

/// Summaryをシリアライズする際のフィールド名です。
const SUMMARY_FIELDS: &'static [&'static str] = &["date", "sum"];
/// SummaryContentをシリアライズする際のフィールド名です。
const SUMMARYCONTENT_FIELDS: &'static [&'static str] = &["data", "last_update"];

/// Summary構造体のフィールド名です。
enum SummaryField {
	Data,
	LastUpdate,
}

// 小計を列挙する構造体です。
#[derive(Debug)]
pub struct Summary {
	pub data: Vec<SummaryContent>,
	pub last_update: DateTime<Local>
}

/// SummaryContent構造体のフィールド名です。
enum SummaryContentField {
	Date,
	Sum
}

/// 小計を格納する構造体です。
#[derive(Debug)]
pub struct SummaryContent {
	pub date: DateTime<Utc>,
	pub sum: u32
}

/// Summaryのシリアライズ処理の実装です。
impl Serialize for Summary {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// 構造体のシリアライズを開始する
		let mut state = serializer.serialize_struct(
			"Summary", 1
		).unwrap();
		// dateフィールドをシリアライズする
		state.serialize_field("date", &self.data).unwrap();
		// last_updateフィールドをシリアライズする
		state.serialize_field(
			"last_update",
			&format!(
				"{}",
				self.last_update.format(FORMAT).to_string()
			)
		).unwrap();
		// ステートを終了し、結果を返却する
		state.end()
	}

}

/// SummaryFieldのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct SummaryFieldVisitor;

impl<'de> Visitor<'de> for SummaryFieldVisitor {

	type Value = SummaryField;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "`data` or `last_update` not found")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: DeserializationError,
	{	
		match value {
			"data" => Ok(SummaryField::Data),
			"last_update" => Ok(SummaryField::LastUpdate),
			_ => Err(DeserializationError::unknown_field(value, SUMMARY_FIELDS))
		}
	}

}

impl<'de> Deserialize<'de> for SummaryField {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(SummaryFieldVisitor)
	}

}

/// SummaryのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct SummaryVisitor;

impl<'de> Visitor<'de> for SummaryVisitor {

	// 変換する対象の構造体型を定義
	type Value = Summary;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "format is not correct.")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, <M as MapAccess<'de>>::Error>
	where
		M: MapAccess<'de>
	{
		let mut data = None;
		let mut last_update = None;

		// 連想配列のキーを取得する
		while let Some(key) = map.next_key::<SummaryField>().unwrap() {
			match key {
				// SummaryContentの可変長配列を取り出す
				SummaryField::Data => {
					// 既にdataに内容が含まれていないか判定
					if data.is_some() {
						return Err(DeserializationError::duplicate_field(SUMMARY_FIELDS[0]));
					}
					// SummaryContentの可変長配列をパースし、格納する
					data = Some(
						map.next_value::<Vec<SummaryContent>>().unwrap()
					);
				},
				SummaryField::LastUpdate => {
					// 既にlast_updateに内容が含まれていないか判定
					if last_update.is_some() {
						return Err(DeserializationError::duplicate_field(SUMMARY_FIELDS[0]));
					}
					// 日付と時刻をパースし、格納する
					last_update = Some(
						Local.datetime_from_str(&map.next_value::<String>().unwrap(), FORMAT)
						.unwrap()
					);
				}
			}
		}

		// dataの中身を取り出す
		let data = data.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(SUMMARY_FIELDS[0])
		)?;
		// last_updateの中身を取り出す
		let last_update = last_update.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(SUMMARY_FIELDS[1])
		)?;

		// Summaryを返却
		Ok(Summary { data: data, last_update: last_update })
	}

}

impl<'de> Deserialize<'de> for Summary {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_struct("Summary", SUMMARY_FIELDS, SummaryVisitor)
	}

}

/// SummaryContentのシリアライズ処理の実装です。
impl Serialize for SummaryContent {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// 構造体のシリアライズを開始する
		let mut state = serializer.serialize_struct(
			"SummaryContent", 1
		).unwrap();
		// dateフィールドをシリアライズする
		state.serialize_field("date", &self.date.to_rfc3339()).unwrap();
		// sumフィールドをシリアライズする
		state.serialize_field("sum", &self.sum).unwrap();
		// ステートを終了し、結果を返却する
		state.end()
	}

}

/// PatientsFieldのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct SummaryContentFieldVisitor;

impl<'de> Visitor<'de> for SummaryContentFieldVisitor {

	type Value = SummaryContentField;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "`date` or `sum` not found")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: DeserializationError,
	{	
		match value {
			"date" => Ok(SummaryContentField::Date),
			"sum" => Ok(SummaryContentField::Sum),
			_ => Err(DeserializationError::unknown_field(value, SUMMARYCONTENT_FIELDS))
		}
	}

}

impl<'de> Deserialize<'de> for SummaryContentField {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(SummaryContentFieldVisitor)
	}

}

/// SummaryContentのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct SummaryContentVisitor;

impl<'de> Visitor<'de> for SummaryContentVisitor {

	// 変換する対象の構造体型を定義
	type Value = SummaryContent;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "format is not correct.")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, <M as MapAccess<'de>>::Error>
	where
		M: MapAccess<'de>
	{
		let mut date = None;
		let mut sum = None;

		// 連想配列のキーを取得する
		while let Some(key) = map.next_key::<SummaryContentField>().unwrap() {
			match key {
				// dateを取り出す
				SummaryContentField::Date => {
					// 既にdateに内容が含まれていないか判定
					if date.is_some() {
						return Err(DeserializationError::duplicate_field(SUMMARYCONTENT_FIELDS[0]));
					}
					// 日付と時刻をパースし、格納する
					date = Some(
						map.next_value::<String>().unwrap().parse::<DateTime<Utc>>().unwrap()
					);
				},
				SummaryContentField::Sum => {
					// 既にsumに内容が含まれていないか判定
					if sum.is_some() {
						return Err(DeserializationError::duplicate_field(SUMMARYCONTENT_FIELDS[1]));
					}
					// 整数値をパースし、格納する
					sum = Some(
						map.next_value::<u32>().unwrap()
					);
				}
			}
		}

		// dateの中身を取り出す
		let date = date.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(SUMMARYCONTENT_FIELDS[0])
		)?;
		// sumの中身を取り出す
		let sum = sum.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(SUMMARYCONTENT_FIELDS[1])
		)?;

		// Patientsを返却
		Ok(SummaryContent { date: date, sum: sum })
	}

}

impl<'de> Deserialize<'de> for SummaryContent {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_struct(
			"SummaryContent",
			SUMMARYCONTENT_FIELDS,
			SummaryContentVisitor
		)
	}

}
