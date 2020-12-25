use chrono::NaiveDate;
use chrono::prelude::{Local, Utc};

// fn ex1() {
//     println!("Hello, world!"); 
//     
//     let args =  std::env::args();
//     // let a0: <{unknown} as Index<i32>>::Output = args[0];
// 
//     let collectedArgs: Vec<String> = args.collect();
//     let collectedArg0 = collectedArgs[0];
// }
// 
// fn today_as_text() {
//     let now = Local.offset_from_local_datetime(local)
//     let today = Local.from_local_date(local: &NaiveDate)
// }

// Output:
// [local time]     2021-02-05+09:00 to_string()
// [local time]     2021-02-05+09:00 "{}"
// [local time]     2021-02-05+09:00 "{:?}"
fn print_local_today() {
    let today = Local::today();
    let s = today.to_string();
    println!("[local time] \t {} to_string()", s);
    println!("[local time] \t {} \"{{}}\"", today);
    println!("[local time] \t {} \"{{:?}}\"", today);
}

// Output:
// [utc]    2021-02-04UTC to_string()
// [utc]    2021-02-04UTC "{}"
// [utc]    2021-02-04UTC "{:?}"
fn print_utc_today() {
    let today = Utc::today();
    let s = today.to_string();
    println!("[utc] \t {} to_string()", s);
    println!("[utc] \t {} \"{{}}\"", today);
    println!("[utc] \t {} \"{{:?}}\"", today);
}

fn parse_from_datestring() -> Result<NaiveDate, chrono::ParseError> {
    let datestr = "2021-02-04";
    let date_obj = NaiveDate::parse_from_str(datestr, "%Y-%m-%d");
    return date_obj;
}

fn main() {
    print_local_today();
    print_utc_today();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_today() {
        print_local_today();
        print_utc_today();
    }

}
