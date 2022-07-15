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
	Local,
	TimeZone
};
use crate::utils::formats::FORMAT;

/// シリアライズする際のフィールド名です。
const FIELDS: &'static [&'static str] = &["update_data"];

/// LastUpdate構造体のフィールド名です。
enum LastUpdateField {
	DateTime
}

/// データの最終更新日を格納します。
#[derive(Debug)]
pub struct LastUpdate<Local: TimeZone> {
	/// chronoクレートの`DateTime<Local>`型の値を格納します。
	pub datetime: chrono::DateTime<Local>
}

/// シリアライズ処理の実装です。
impl Serialize for LastUpdate<Local> {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// 構造体のシリアライズを開始する
		let mut state = serializer.serialize_struct(
			"LastUpdate", 1
		).unwrap();
		// datetimeフィールドをシリアライズする
		state.serialize_field(
			"last_update",
			&format!(
				"{}",
				self.datetime.format(FORMAT).to_string()
			)
		).unwrap();
		// ステートを終了し、結果を返却する
		state.end()
	}

}

/// LastUpdateFieldのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct LastUpdateFieldVisitor;

impl<'de> Visitor<'de> for LastUpdateFieldVisitor {

	type Value = LastUpdateField;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "`last_update` not found")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: DeserializationError,
	{	
		match value {
			"last_update" => Ok(LastUpdateField::DateTime),
			_ => Err(DeserializationError::unknown_field(value, FIELDS))
		}
	}

}

impl<'de> Deserialize<'de> for LastUpdateField {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(LastUpdateFieldVisitor)
	}

}

/// LastUpdateのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct LastUpdateVisitor;

impl<'de> Visitor<'de> for LastUpdateVisitor {

	// 変換する対象の構造体型を定義
	type Value = LastUpdate<Local>;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "format is not correct.")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, <M as MapAccess<'de>>::Error>
	where
		M: MapAccess<'de>
	{
		let mut update_date = None;

		// 連想配列のキーを取得し
		while let Some(key) = map.next_key::<LastUpdateField>().unwrap() {
			match key {
				// DateTimeを取り出し
				LastUpdateField::DateTime => {
					// 既にupdate_dateに内容が含まれていないか判定
					if update_date.is_some() {
						return Err(DeserializationError::duplicate_field(FIELDS[0]));
					}
					// 日付と時刻をパースし、格納する
					update_date = Some(
						Local.datetime_from_str(&map.next_value::<String>().unwrap(), FORMAT)
						.unwrap()
					);
				}
			}
		}

		// update_dateの中身を取り出す
		let update_date = update_date.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(FIELDS[0])
		)?;

		// LastUpdateを返却
		Ok(LastUpdate { datetime: update_date })
	}

}

impl<'de> Deserialize<'de> for LastUpdate<Local> {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_struct("LastUpdate", FIELDS, LastUpdateVisitor)
	}

}
