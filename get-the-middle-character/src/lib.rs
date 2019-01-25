// https://www.codewars.com/kata/get-the-middle-character/

#[allow(dead_code)]
fn get_middle(s:&str) -> &str {
    if s.len() % 2 == 0 {
        &s[s.len()/2-1..s.len()/2+1]
    } else {
        &s[s.len()/2..s.len()/2+1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(get_middle("test"),"es");
        assert_eq!(get_middle("testing"),"t");
        assert_eq!(get_middle("middle"),"dd");
        assert_eq!(get_middle("A"),"A");
        assert_eq!(get_middle("of"),"of");
    }
}
