// https://www.codewars.com/kata/sequencesum/

#[allow(dead_code)]
fn sum_of_n(n: i32) -> Vec<i32> {
    let f = if n < 0 { -1 } else { 1 };
    (0..n.abs()+1).map(|i| f * i * (i + 1) / 2).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_tests() {
        assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
        assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
        assert_eq!(sum_of_n(1), vec![0, 1]);
        assert_eq!(sum_of_n(0), vec![0]);
        assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
    }
}
