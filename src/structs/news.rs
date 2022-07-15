use chrono::NaiveDate;
use serde::{
    de::{Error as DeserializationError, MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Error, Formatter};

use crate::utils::formats::DATE_FORMAT;

///  NewsItemをシリアライズする際のフィールド名です。
const NEWS_ITEM_FIELDS: &'static [&'static str] = &["date", "text", "url"];

#[derive(Clone, Debug)]
/// NewsItem構造体のフィールド名です。
enum NewsItemField {
    Date,
    Text,
    Url,
}

#[derive(Clone, Debug)]
pub struct NewsItem {
    pub date: NaiveDate,
    pub text: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewsItems {
    pub news_items: Vec<NewsItem>,
}

/// NewsItemのシリアライズ処理の実装です。
impl Serialize for NewsItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 構造体のシリアライズを開始する
        let mut state = serializer.serialize_struct("NewsItem", 1).unwrap();
        // dateフィールドをシリアライズする
        state
            .serialize_field(
                "date",
                &format!("{}", &self.date.format(DATE_FORMAT).to_string()),
            )
            .unwrap();
        // textフィールドをシリアライズする
        state.serialize_field("text", &self.text).unwrap();
        // urlフィールドをシリアライズする
        state.serialize_field("url", &self.url).unwrap();
        // ステートを終了し、結果を返却する
        state.end()
    }
}

/// PatientsFieldのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct NewsItemFieldVisitor;

impl<'de> Visitor<'de> for NewsItemFieldVisitor {
    type Value = NewsItemField;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "`date`, `text` or `url` not found")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializationError,
    {
        match value {
            "date" => Ok(NewsItemField::Date),
            "text" => Ok(NewsItemField::Text),
            "url" => Ok(NewsItemField::Url),
            _ => Err(DeserializationError::unknown_field(value, NEWS_ITEM_FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for NewsItemField {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(NewsItemFieldVisitor)
    }
}

/// NewsItemのVisitorを定義します。
///
/// ※この構造体は、Visitorトレイトを実装することを意図しています。
struct NewsItemVisitor;

impl<'de> Visitor<'de> for NewsItemVisitor {
    // 変換する対象の構造体型を定義
    type Value = NewsItem;

    fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "format is not correct.")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, <M as MapAccess<'de>>::Error>
    where
        M: MapAccess<'de>,
    {
        let mut date = None;
        let mut text = None;
        let mut url = None;

        // 連想配列のキーを取得する
        while let Some(key) = map.next_key::<NewsItemField>().unwrap() {
            match key {
                // dateを取り出す
                NewsItemField::Date => {
                    // 既にdateに内容が含まれていないか判定
                    if date.is_some() {
                        return Err(DeserializationError::duplicate_field(NEWS_ITEM_FIELDS[0]));
                    }
                    // 日付と時刻をパースし、格納する
                    date = Some(
                        NaiveDate::parse_from_str(
                            &map.next_value::<String>().unwrap(),
                            DATE_FORMAT,
                        )
                        .unwrap(),
                    );
                }
                NewsItemField::Text => {
                    // 既にtextに内容が含まれていないか判定
                    if text.is_some() {
                        return Err(DeserializationError::duplicate_field(NEWS_ITEM_FIELDS[1]));
                    }
                    // Stringをパースし、格納する
                    text = Some(map.next_value::<String>().unwrap());
                }
                NewsItemField::Url => {
                    // 既にurlに内容が含まれていないか判定
                    if url.is_some() {
                        return Err(DeserializationError::duplicate_field(NEWS_ITEM_FIELDS[2]));
                    }
                    // Stringをパースし、格納する
                    url = Some(map.next_value::<String>().unwrap());
                }
            }
        }

        // dateの中身を取り出す
        let date = date.ok_or_else(
            // フィールドが不足していることを伝える
            || DeserializationError::missing_field(NEWS_ITEM_FIELDS[0]),
        )?;
        // textの中身を取り出す
        let text = text.ok_or_else(
            // フィールドが不足していることを伝える
            || DeserializationError::missing_field(NEWS_ITEM_FIELDS[1]),
        )?;
        // urlの中身を取り出す
        let url = url.ok_or_else(
            // フィールドが不足していることを伝える
            || DeserializationError::missing_field(NEWS_ITEM_FIELDS[2]),
        )?;

        // Patientsを返却
        Ok(NewsItem {
            date: date,
            text: text,
            url: url,
        })
    }
}

impl<'de> Deserialize<'de> for NewsItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("NewsItem", NEWS_ITEM_FIELDS, NewsItemVisitor)
    }
}
