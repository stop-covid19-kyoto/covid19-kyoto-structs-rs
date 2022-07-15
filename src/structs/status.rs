use chrono::{DateTime, Local, TimeZone};
use serde::{
    de::{Error as DeserializationError, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Error, Formatter};

use crate::utils::formats::DATETIME_FORMAT;

/// Statusをシリアライズする際のフィールド名です。
const STATUS_FIELDS: &'static [&'static str] = &["attr", "value", "children", "last_update"];

/// COVID-19に関連する情報の属性を列挙しています。
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Attributes {
    /// 宿泊施設で療養している人数の属性です。
    #[serde(rename = "accommodations")]
    Accommodations,
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
    #[serde(rename = "severely_patients")]
    SeverelyPatients,
    /// 重症化のうち、他の方法による対応を受けている人数の属性です。
    #[serde(rename = "other")]
    Other,
}

enum StatusField {
    Attr,
    Value,
    Children,
    LastUpdate,
}

/// COVID-19に関する情報を格納する構造体です。
#[derive(Clone, Debug)]
pub struct Status {
    pub attr: Attributes,
    pub value: u32,
    pub children: Option<Vec<Status>>,
    pub last_update: Option<DateTime<Local>>,
}

/// Summaryのシリアライズ処理の実装です。
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 構造体のシリアライズを開始する
        let mut state = serializer.serialize_struct("Summary", 1).unwrap();
        // attrフィールドをシリアライズする
        state.serialize_field("attr", &self.attr).unwrap();
        // valueフィールドをシリアライズする
        state.serialize_field("value", &self.value).unwrap();
        // childrenフィールドをシリアライズする
        match &self.children {
            Some(v) => {
                state.serialize_field("children", v).unwrap();
            }
            None => (),
        }
        // last_updateフィールドをシリアライズする
        match self.last_update {
            Some(v) => {
                state
                    .serialize_field(
                        "last_update",
                        &format!("{}", v.format(DATETIME_FORMAT).to_string()),
                    )
                    .unwrap();
            }
            None => (),
        }
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
        write!(
            formatter,
            "`attr`, `value`, `children` or `last_update` not found"
        )
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializationError,
    {
        match value {
            "attr" => Ok(StatusField::Attr),
            "value" => Ok(StatusField::Value),
            "children" => Ok(StatusField::Children),
            "last_update" => Ok(StatusField::LastUpdate),
            _ => Err(DeserializationError::unknown_field(value, STATUS_FIELDS)),
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
        M: MapAccess<'de>,
    {
        let mut attr = None;
        let mut value = None;
        let mut children = None;
        let mut last_update = None;

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
                }
                // 整数値を取り出す
                StatusField::Value => {
                    // 既にvalueに内容が含まれていないか判定
                    if value.is_some() {
                        return Err(DeserializationError::duplicate_field(STATUS_FIELDS[1]));
                    }
                    // 整数値をパースし、格納する
                    value = Some(map.next_value::<u32>().unwrap());
                }
                // 子属性を取り出す
                StatusField::Children => {
                    // 既にchildrenに内容が含まれていないか判定
                    if children.is_some() {
                        return Err(DeserializationError::duplicate_field(STATUS_FIELDS[2]));
                    }
                    // 子属性の内容をパースし、格納する
                    children = Some(map.next_value::<Vec<Status>>().unwrap());
                }
                // 最終更新日時を取り出す
                StatusField::LastUpdate => {
                    // 既にlast_updateに内容が含まれていないか判定
                    if last_update.is_some() {
                        return Err(DeserializationError::duplicate_field(STATUS_FIELDS[3]));
                    }
                    // last_updateの内容をパースし、格納する
                    last_update = Some(
                        Local
                            .datetime_from_str(
                                &map.next_value::<String>().unwrap(),
                                DATETIME_FORMAT,
                            )
                            .unwrap(),
                    );
                }
            }
        }

        // attrの中身を取り出す
        let attr = attr.ok_or_else(
            // フィールドが不足していることを伝える
            || DeserializationError::missing_field(STATUS_FIELDS[0]),
        )?;
        // valueの中身を取り出す
        let value = value.ok_or_else(
            // フィールドが不足していることを伝える
            || DeserializationError::missing_field(STATUS_FIELDS[1]),
        )?;
        // childrenの中身を取り出す
        let children = children;
        // last_updateの中身を取り出す
        let last_update = last_update;

        // Summaryを返却
        Ok(Status {
            attr: attr,
            value: value,
            children: children,
            last_update: last_update,
        })
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
