pub mod index;

pub mod adventures;

pub mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
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
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod my_item_type_format {

    const DAILY: &'static str = "日常";
    const HAHA: &'static str = "搞笑";
    const GAME: &'static str = "游戏";
    const FILM_TV: &'static str = "影视";
    const JOURNEY: &'static str = "旅游";
    const FOOD: &'static str = "美食";
    const NONE: &'static str = "";

    pub fn to_item_type_name(item_type: u8) -> String {
        match item_type {
            1 => DAILY.to_owned(),
            2 => HAHA.to_owned(),
            3 => GAME.to_owned(),
            4 => FILM_TV.to_owned(),
            5 => JOURNEY.to_owned(),
            6 => FOOD.to_owned(),
            _ => NONE.to_owned(),
        }
    }
}

pub mod my_source {
    const DOUYIN: &'static str = "抖音";
    const BILIBILI: &'static str = "哔哩哔哩";
    const XIGUASHIPIN: &'static str = "西瓜视频";
    const NONE: &'static str = "";

    pub fn to_source_name(source: u8) -> String {
        match source {
            1 => DOUYIN.to_owned(),
            2 => BILIBILI.to_owned(),
            3 => XIGUASHIPIN.to_owned(),
            _ => NONE.to_owned(),
        }
    }
}

pub mod my_journey_destiny {
    // 省
    const HENAN: &'static str = "河南";
    const HEBEI: &'static str = "河北";
    const SHANXI_JIN: &'static str = "山西";
    const SHANDONG: &'static str = "山东";
    const GUANGXI: &'static str = "广西";
    const GUANGDONG: &'static str = "广东";
    const FUJIAN: &'static str = "福建";
    const ANHUI: &'static str = "安徽";
    const GUIZHOU: &'static str = "贵州";
    const HAINAN: &'static str = "海南";
    const HUNAN: &'static str = "湖南";
    const HUBEI: &'static str = "湖北";
    const JIANGXI: &'static str = "江西";
    const ZHEJIANG: &'static str = "浙江";
    const SICHUAN: &'static str = "四川";
    const XIZANG: &'static str = "西藏";
    const QINGHAI: &'static str = "青海";
    const XINJIANG: &'static str = "新疆";
    const GANSU: &'static str = "甘肃";
    const NINGXIA: &'static str = "宁夏";
    const NEIMENGGU: &'static str = "内蒙古";
    const SHANXI_SHAN: &'static str = "陕西";
    const JIANGSU: &'static str = "江苏";
    const HEILONGJIANG: &'static str = "黑龙江";
    const JILIN: &'static str = "吉林";
    const LIAONING: &'static str = "辽宁";
    const YUNNAN: &'static str = "云南";
    // 直辖市
    const CHONGQING: &'static str = "重庆";
    const BEIJING: &'static str = "北京";
    const SHANGHAI: &'static str = "上海";
    const TIANJIN: &'static str = "天津";
    // 港澳台
    const XIANGGANG: &'static str = "香港";
    const AOMEN: &'static str = "澳门";
    const TAIWAN: &'static str = "台湾";

    const NONE: &'static str = "";

    pub fn to_name(code: &str) -> String {
        match code {
            "henan" => HENAN.to_owned(),
            "hebei" => HEBEI.to_owned(),
            "shanxi_jin" => SHANXI_JIN.to_owned(),
            "shandong" => SHANDONG.to_owned(),
            "guangxi" => GUANGXI.to_owned(),
            "guangdong" => GUANGDONG.to_owned(),
            "fujian" => FUJIAN.to_owned(),
            "anhui" => ANHUI.to_owned(),
            "guizhou" => GUIZHOU.to_owned(),
            "hainan" => HAINAN.to_owned(),
            "hunan" => HUNAN.to_owned(),
            "hubei" => HUBEI.to_owned(),
            "jiangxi" => JIANGXI.to_owned(),
            "zhejiang" => ZHEJIANG.to_owned(),
            "sichuan" => SICHUAN.to_owned(),
            "qinghai" => QINGHAI.to_owned(),
            "xizang" => XIZANG.to_owned(),
            "xinjiang" => XINJIANG.to_owned(),
            "gansu" => GANSU.to_owned(),
            "ningxia" => NINGXIA.to_owned(),
            "neimenggu" => NEIMENGGU.to_owned(),
            "shanxi_shan" => SHANXI_SHAN.to_owned(),
            "jiangsu" => JIANGSU.to_owned(),
            "heilongjiang" => HEILONGJIANG.to_owned(),
            "jilin" => JILIN.to_owned(),
            "liaoning" => LIAONING.to_owned(),
            "yunnan" => YUNNAN.to_owned(),

            "chongqing" => CHONGQING.to_owned(),
            "beijing" => BEIJING.to_owned(),
            "shanghai" => SHANGHAI.to_owned(),
            "tianjin" => TIANJIN.to_owned(),

            "xianggang" => XIANGGANG.to_owned(),
            "aomen" => AOMEN.to_owned(),
            "taiwan" => TAIWAN.to_owned(),
            _ => NONE.to_owned(),
        }
    }
}
