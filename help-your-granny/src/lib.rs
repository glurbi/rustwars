// https://www.codewars.com/kata/help-your-granny/

use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! hash_map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[allow(dead_code)]
fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
    let mut d: f64 = 0.0;
    let mut it = frnds.iter();
    let mut x1 = fr_twns[*it.next().unwrap()];
    let mut d1 = dist[x1];
    d += d1;
    loop {
        match it.next() {
            Some(f) => {
                if !fr_twns.contains_key(f) {
                    continue;
                }
                let x2 = fr_twns[*f];
                let d2 = dist[x2];
                d += (d2 * d2 - d1 * d1).sqrt();
                d1 = d2;
                x1 = x2;
            },
            None => break,
        }
    }
    d += dist[x1];
    d.floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(frnds:  &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>, exp: i32) -> () {
        assert_eq!(tour(&frnds, fr_twns, dist), exp)
    }

    #[test]
    fn tests_tour() {
        let friends = [ "A1", "A2", "A3", "A4", "A5" ];
        let fr_towns = hash_map!{ "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" }; 
        let dst = hash_map!{ "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
        testing(&friends, fr_towns, dst, 889);
    }
}
