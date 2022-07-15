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

/// Summaryをシリアライズする際のフィールド名です。
const DETAILEDSTATUS_FIELDS: &'static [&'static str] = &["attr", "value"];
/// SummaryContentをシリアライズする際のフィールド名です。
const STATUS_FIELDS: &'static [&'static str] = &["attr", "value", "children"];

/// COVID-19に関連する情報の属性を列挙しています。
#[derive(Debug, Deserialize, Serialize)]
pub enum Attributes {
	/// 宿泊施設で療養している人数の属性です。
	#[serde(rename = "accomendations")]
	Acommendations,
	/// 医療・行政機関等により調整作業を行なっている人数の属性です。
	#[serde(rename = "coodinating")]
	Coodinating,
	/// COVID-19によりお亡くなりになられた人数の属性です。
	#[serde(rename = "dead")]
	Dead,
	/// 自宅療養中の人数の属性です。
	#[serde(rename = "home")]
	Home,
	/// 入院中の人数の属性です。
	#[serde(rename = "hospitalizations")]
	Hospitalizations,
	/// PCR検査件数の属性です。
	#[serde(rename = "inspections")]
	Inspections,
	/// 退院した人数の属性です。
	#[serde(rename = "leave")]
	Leave,
	/// 陽性者数の属性です。
	#[serde(rename = "patients")]
	Patients,
	/// 症状の重症化により、高度重症病床を利用されている人数の属性です。
	#[serde(rename = "severepatients")]
	SeverePatients,
	/// 重症化のうち、他の方法による対応を受けている人数の属性です。
	#[serde(rename = "other")]
	Other,
}

enum DetailedStatusField {
	Attr,
	Value
}

/// COVID-19に関する情報を格納する構造体です。
#[derive(Debug)]
pub struct DetailedStatus {
	pub attr: Attributes,
	pub value: u32
}

enum StatusField {
	Attr,
	Value,
	Children
}

/// COVID-19に関する情報を、子属性と共に格納する構造体です。
#[derive(Debug)]
pub struct Status {
	pub attr: Attributes,
	pub value: u32,
	pub children: Vec<DetailedStatus>
}

/// DetailedStatusのシリアライズ処理の実装です。
impl Serialize for DetailedStatus {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// 構造体のシリアライズを開始する
		let mut state = serializer.serialize_struct(
			"DetailedStatus", 1
		).unwrap();
		// attrフィールドをシリアライズする
		state.serialize_field("attr", &self.attr).unwrap();
		// valueフィールドをシリアライズする
		state.serialize_field("value", &self.value).unwrap();
		// ステートを終了し、結果を返却する
		state.end()
	}

}

/// DetailedStatusFieldのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct DetailedStatusFieldVisitor;

impl<'de> Visitor<'de> for DetailedStatusFieldVisitor {

	type Value = DetailedStatusField;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "`attr` or `value` not found")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: DeserializationError,
	{	
		match value {
			"attr" => Ok(DetailedStatusField::Attr),
			"value" => Ok(DetailedStatusField::Value),
			_ => Err(DeserializationError::unknown_field(value, DETAILEDSTATUS_FIELDS))
		}
	}

}

impl<'de> Deserialize<'de> for DetailedStatusField {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(DetailedStatusFieldVisitor)
	}

}

/// DetailedStatusのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct DetailedStatusVisitor;

impl<'de> Visitor<'de> for DetailedStatusVisitor {

	// 変換する対象の構造体型を定義
	type Value = DetailedStatus;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "format is not correct.")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, <M as MapAccess<'de>>::Error>
	where
		M: MapAccess<'de>
	{
		let mut attr = None;
		let mut value = None;

		// 連想配列のキーを取得する
		while let Some(key) = map.next_key::<DetailedStatusField>().unwrap() {
			match key {
				// Attributesを取り出す
				DetailedStatusField::Attr => {
					// 既にattrに内容が含まれていないか判定
					if attr.is_some() {
						return Err(DeserializationError::duplicate_field(DETAILEDSTATUS_FIELDS[0]));
					}
					// Attributesをパースし、格納する
					attr = Some(map.next_value::<Attributes>().unwrap());
				},
				// Valueを取り出す
				DetailedStatusField::Value => {
					// 既にvalueに内容が含まれていないか判定
					if value.is_some() {
						return Err(DeserializationError::duplicate_field(DETAILEDSTATUS_FIELDS[0]));
					}
					// 整数値をパースし、格納する
					value = Some(map.next_value::<u32>().unwrap());
				}
			}
		}

		// attrの中身を取り出す
		let attr = attr.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(DETAILEDSTATUS_FIELDS[0])
		)?;
		// valueの中身を取り出す
		let value = value.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(DETAILEDSTATUS_FIELDS[1])
		)?;

		// DetailedStatusを返却
		Ok(DetailedStatus { attr: attr, value: value })
	}

}

impl<'de> Deserialize<'de> for DetailedStatus {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_struct("DetailedStatus", DETAILEDSTATUS_FIELDS, DetailedStatusVisitor)
	}

}

/// Statusのシリアライズ処理の実装です。
impl Serialize for Status {

	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// 構造体のシリアライズを開始する
		let mut state = serializer.serialize_struct(
			"Status", 1
		).unwrap();
		// attrフィールドをシリアライズする
		state.serialize_field("attr", &self.attr).unwrap();
		// valueフィールドをシリアライズする
		state.serialize_field("value", &self.value).unwrap();
		// childrenフィールドをシリアライズする
		state.serialize_field("children", &self.children).unwrap();
		// ステートを終了し、結果を返却する
		state.end()
	}

}

/// StatusFieldのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct StatusFieldVisitor;

impl<'de> Visitor<'de> for StatusFieldVisitor {

	type Value = StatusField;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "`attr`, `value` or `children` not found")
	}

	fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
	where
		E: DeserializationError,
	{	
		match value {
			"attr" => Ok(StatusField::Attr),
			"value" => Ok(StatusField::Value),
			"children" => Ok(StatusField::Children),
			_ => Err(DeserializationError::unknown_field(value, STATUS_FIELDS))
		}
	}

}

impl<'de> Deserialize<'de> for StatusField {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(StatusFieldVisitor)
	}

}

/// StatusのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct StatusVisitor;

impl<'de> Visitor<'de> for StatusVisitor {

	// 変換する対象の構造体型を定義
	type Value = Status;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		write!(formatter, "format is not correct.")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, <M as MapAccess<'de>>::Error>
	where
		M: MapAccess<'de>
	{
		let mut attr = None;
		let mut value = None;
		let mut children = None;

		// 連想配列のキーを取得する
		while let Some(key) = map.next_key::<StatusField>().unwrap() {
			match key {
				// Attributesを取り出す
				StatusField::Attr => {
					// 既にattrに内容が含まれていないか判定
					if attr.is_some() {
						return Err(DeserializationError::duplicate_field(STATUS_FIELDS[0]));
					}
					// Attributesをパースし、格納する
					attr = Some(map.next_value::<Attributes>().unwrap());
				},
				// 整数値を取り出す
				StatusField::Value => {
					// 既にvalueに内容が含まれていないか判定
					if value.is_some() {
						return Err(DeserializationError::duplicate_field(STATUS_FIELDS[1]));
					}
					// 整数値をパースし、格納する
					value = Some(map.next_value::<u32>().unwrap());
				},
				// 子属性を取り出す
				StatusField::Children => {
					// 既にchildrenに内容が含まれていないか判定
					if children.is_some() {
						return Err(DeserializationError::duplicate_field(STATUS_FIELDS[2]));
					}
					// 子属性の内容をパースし、格納する
					children = Some(map.next_value::<Vec<DetailedStatus>>().unwrap());
				}
			}
		}

		// attrの中身を取り出す
		let attr = attr.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(STATUS_FIELDS[0])
		)?;
		// valueの中身を取り出す
		let value = value.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(STATUS_FIELDS[1])
		)?;
		// childrenの中身を取り出す
		let children = children.ok_or_else(
			// フィールドが不足していることを伝える
			|| DeserializationError::missing_field(STATUS_FIELDS[2])
		)?;

		// Summaryを返却
		Ok(Status { attr: attr, value: value, children: children })
	}

}

impl<'de> Deserialize<'de> for Status {

	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_struct("Status", STATUS_FIELDS, StatusVisitor)
	}

}
