#[derive(Debug)]
struct Store {
    id: i32,
    name: String,
    min_hour: i32,
    max_hour: i32,
    interval_minute: i32,
    min_work_time: i32,
}

// fn workable_minutes_range(s: &Store) -> std::ops::RangeInclusive<i32> {
//     let min: i32 = s.min_hour * 60;
//     let max: i32 = s.max_hour * 60;
//     return min..=max;
// }

// use std::convert::TryFrom;
use std::ops::Range;
use std::iter::StepBy;

impl Store {
    #[allow(dead_code)]
    pub fn workable_minutes_range(&self) -> StepBy<Range<i32>> {
        let range = (self.min_hour * 60)..(self.max_hour * 60 + 1);
        let step_seconds = self.interval_minute as usize;
        let stepby = range.step_by(step_seconds);
        return stepby;
    }
}

#[allow(dead_code)]
fn to_hhmm(as_min: i32) -> String {
    let h = as_min / 60;
    let m = as_min % 60;
    return format!("{:0>2}:{:0>2}", h , m);
}

#[derive(Debug, PartialEq)]
// * derive(PartialOrd) というのもできる
// * 複数回にわけてもよい
//   #[derive(Debug)]
//   #[derive(PartialEq)]
struct Pair (i32, i32);


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_store1() -> Store {
        return Store { 
            id: 10, 
            name: "xxx".to_string(),
            min_hour: 10,
            max_hour: 48,
            interval_minute: 30,
            min_work_time: 60,
        };
    }

    #[test]
    fn test_new_store() {
        assert_eq!(sample_store1().name, "xxx");
    }

    #[test]
    fn test_debug_format() {
        let expect = "Store { id: 10, name: \"xxx\", min_hour: 10, max_hour: 48, interval_minute: 30, min_work_time: 60 }";
        let got = format!("{:?}", sample_store1());
        assert_eq!(got, expect);
    }

    #[test]
    fn test_pair() {
        assert_eq!(Pair(39, 2), Pair(39, 2));
    }

    #[test]
    fn tes_1() {
        let store1 = sample_store1();
        let store2 = Store {
            min_hour: 2, 
            max_hour: 4, 
            ..store1
        };
        let r: StepBy<Range<i32>> = store2.workable_minutes_range();
        let workable_time_vec: Vec<i32> = r.collect();
        assert_eq!(workable_time_vec, [120, 150, 180, 210, 240]);
    }

    #[test]
    fn test_hhmm() {
        assert_eq!(to_hhmm(540), "09:00");
    }
}