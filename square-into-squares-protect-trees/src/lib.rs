// https://www.codewars.com/kata/square-into-squares-protect-trees/

#[allow(dead_code)]
fn decompose(n: i64) -> Option<Vec<i64>> {
    //println!("n:{}", n);
    let mut v: Vec<i64> = vec![];
    match decompose_rec(n, n, &mut v) {
        Some(v) => Some(v.iter().rev().map(|x| *x).collect()),
        None => None,
    }
}

fn decompose_rec(n: i64, m: i64, v: &mut Vec<i64>) -> Option<Vec<i64>> {
    //println!("n:{} m:{} v:{:?}", n, m, v);
    match sum_square(v) {
        sum if sum == n*n && m >= 0 => return Some(v.clone()),
        sum if sum > n*n => return None,
        _ => (),
    };
    for k in (1..m).rev() {
        //let mut v = v.clone();
        v.push(k);
        match decompose_rec(n, k, v) {
            Some(v) => return Some(v),
            None => { v.pop(); continue; },
        }
    };
    None
}

fn sum_square(v: &Vec<i64>) -> i64 {
    v.iter().map(|x| x*x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
        assert_eq!(decompose(n), exp)
    }

    #[test]
    fn tests_decompose() {
        testing(4, None);
        testing(123123123, Some(vec![1, 3, 5, 11, 85, 15692, 123123122]));
        testing(5, Some(vec![3,4]));
        testing(11, Some(vec![1,2,4,10]));
        testing(50, Some(vec![1,3,5,8,49]));
        testing(44, Some(vec![2,3,5,7,43]));
        testing(625, Some(vec![2,5,8,34,624]));
    }
}
