pub mod articles;
pub mod comments;
pub mod index;
pub mod profiles;
pub mod users;
//
pub mod adventures;

pub mod my_date_format {
    use chrono::{DateTime, FixedOffset, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let tz_offset = FixedOffset::east(8 * 3600);
        tz_offset
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod my_item_type_format {

    const ADVENTURE: &'static str = "探险";
    const HAHA: &'static str = "搞笑";
    const GAME: &'static str = "游戏";
    const FILM_TV: &'static str = "影视";
    const NONE: &'static str = "";

    pub fn to_item_type_name(item_type: u8) -> String {
        match item_type {
            1 => ADVENTURE.to_owned(),
            2 => HAHA.to_owned(),
            3 => GAME.to_owned(),
            4 => FILM_TV.to_owned(),
            _ => NONE.to_owned()
        }
    }
}