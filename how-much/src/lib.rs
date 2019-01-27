// https://www.codewars.com/kata/how-much/

use std::cmp::{min,max};

#[allow(dead_code)]
fn how_much(m: i32, n:i32) -> Vec<(String, String, String)> {
    let mut res = vec![];
    for f in min(m,n)..max(m,n)+1 {
        if f % 9 == 1 && f % 7 == 2 {
            let m = format!("M: {}", f);
            let b = format!("B: {}", f / 7);
            let c = format!("C: {}", f / 9);
            res.push((m, b, c));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(m: i32, n: i32, exp: Vec<(&str, &str, &str)>) -> () {
        let ans: String = format!("{:?}", how_much(m, n));
        let sol: String = format!("{:?}", exp);
        assert_eq!(ans, sol)
    }

    #[test]
    fn basics_how_much() {
        testing(1, 100, vec![("M: 37", "B: 5", "C: 4"), ("M: 100", "B: 14", "C: 11")]);
        testing(1000, 1100, vec![("M: 1045", "B: 149", "C: 116")]);
        testing(10000, 9950, vec![("M: 9991", "B: 1427", "C: 1110")]);
        testing(0, 200, vec![("M: 37", "B: 5", "C: 4"), ("M: 100", "B: 14", "C: 11"), ("M: 163", "B: 23", "C: 18")]);
        testing(2950, 2950, vec![]); 
    }
}
