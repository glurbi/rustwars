// https://www.codewars.com/kata/product-of-consecutive-fib-numbers/

#[allow(dead_code)]
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fib = (0, 1);
    while fib.0 * fib.1 < prod {
        fib = (fib.1, fib.0 + fib.1);
        if fib.0 * fib.1 == prod {
            return (fib.0, fib.1, true)
        }
    }
    (fib.0, fib.1, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }

    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
    }
}
