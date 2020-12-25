#[allow(dead_code)]
fn ex1() {
    let slice1 = "xx yy zz";
    println!("slice1.as_bytes(): {}", slice1.as_bytes()[0]);
}

/* 最初の単語の末尾文字index + 1　を返す. (Test for rust doc) */
pub fn first_word_tail_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
pub fn first_word(s: &str) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_word_tail_index() {
        let s1 = String::from("Hello World");
        assert_eq!(first_word_tail_index(&s1), 5);
    }
    #[test]
    fn test_first_word() {
        let slice1 = first_word("Boy meats girl");
        assert_eq!(slice1, "Boy");
    }
    #[test]
    fn test_str_comp() {
        let slice1 = first_word("Boy meats girl");
        let is_correct = slice1 == "Boy"; // なるほどこれで文字列比較できるんだ
        assert_eq!(is_correct, true);
    }
}
