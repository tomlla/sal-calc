mod yyyy_mm_dd_to_local_date {
    use chrono::{Local, Date, NaiveDate};
    use chrono::TimeZone;
    use serde::{self, de, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Date<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date<Local>, D::Error>
    where D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let x = NaiveDate::parse_from_str(&s, FORMAT);
        let y = x.map_err(|e| de::Error::custom(format!("chrono returned an error: {}", e)));
        let z = y.map(|nd| Local.from_local_date(&nd).unwrap());
        z
    }
}

use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, Date, Local};

#[derive(Serialize, Deserialize, Debug)]
struct Record1 {
    id: i32,
    date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug)]
struct Record2 {
    id: i32,

    #[serde(with="yyyy_mm_dd_to_local_date")]
    date: Date<Local>,
}

fn main() {
    let json_str = r#"
    {
      "id":1,
      "date":"2021-03-15"
    }"#;
    let record1: Record1 = serde_json::from_str(&json_str).unwrap();
    println!("record1: {:?}", record1);

    let record2: Record2 = serde_json::from_str(&json_str).unwrap();
    println!("record2: {:?}", record2)
}
