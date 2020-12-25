// https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html

// use std::collections::HashMap;
// use std::vec::Vec;
// use std::ops::Range;

#[allow(dead_code)]
fn main() {
    println!("hello");
}

// fn largest<T>(list: &[T]) -> () {
//     println!("{}", list.len());
// }

#[allow(dead_code)]
fn print_len<T>(list: &[T]) -> () {
    println!("list.len() --> {}", list.len());
}

#[cfg(test)]
mod tests {
    use std::vec::Vec;
    use super::*;
    // #[test]
    // fn test_largest() {
    //     assert_eq!(0, vec![]);
    //     assert_eq!(1, vec![1]);
    // }
    #[test]
    fn test_print_len() {
        let list_1: Vec<i32> = vec![];
        print_len(&list_1);
        let list_2 = vec![1];
        print_len(&list_2);
    }

}
