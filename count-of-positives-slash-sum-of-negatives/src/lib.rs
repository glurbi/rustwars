// https://www.codewars.com/kata/count-of-positives-slash-sum-of-negatives/

#[allow(dead_code)]
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![]
    }

    let positives = input.iter().filter(|&i| *i > 0).count() as i32;
    let negatives = input.iter().filter(|&i| *i < 0).sum();

    vec![positives, negatives]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
        let expected1 = vec![10, -65];
        assert_eq!(count_positives_sum_negatives(test_data1), expected1);    
    }

}
