// https://www.codewars.com/kata/string-repeat/

#[allow(dead_code)]
fn repeat_str(src: &str, count: usize) -> String {
  src.repeat(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(repeat_str("a", 4), "aaaa");
        assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
        assert_eq!(repeat_str("abc", 2), "abcabc");
    }
}
