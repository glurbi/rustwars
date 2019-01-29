// https://www.codewars.com/kata/moduli-number-system/

#[allow(dead_code)]
fn from_nb_2str(n: i64, sys: Vec<i64>) -> String {
    if !pairwise_co_prime(&sys) {
        return "Not applicable".to_string();
    }

    if sys.iter().fold(1, |acc, x| acc * x) < n {
        return "Not applicable".to_string();
    }

    sys.iter().map(|x| format!("-{}-", n % x)).collect()
}

fn pairwise_co_prime(sys: &Vec<i64>) -> bool {
    for n1 in sys {
        for n2 in sys {
            if n1 != n2 && gcd(*n1, *n2) != 1 {
                return false;
            }
        }
    }
    true
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, sys: Vec<i64>, exp: &str) -> () {
        assert_eq!(&from_nb_2str(n, sys), exp)
    }

    #[test]
    fn basics_from_nb_2str() {
        testing(6, vec![2,3,4], "Not applicable");
        testing(7, vec![2,3], "Not applicable");
        testing(779, vec![8,7,5,3], "-3--2--4--2-");
        testing(187, vec![8,7,5,3], "-3--5--2--1-");
        testing(3450, vec![13,11,7,5,3,2], "-5--7--6--0--0--0-");
    }
}
