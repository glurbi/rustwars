// https://www.codewars.com/kata/strings-mix/

use std::collections::{HashMap,HashSet};

#[allow(dead_code)]
fn mix(s1: &str, s2: &str) -> String {
    let mut hs: HashSet<(char, char, usize)> = HashSet::new();
    populate(s1, '1', &mut hs);
    populate(s2, '2', &mut hs);
    let mut v: Vec<String> = Vec::new();
    while hs.len() > 0  {
        let init = ('?', ' ', 0 as usize);
        let mut max = init;
        for (id, c, n) in hs.iter() {
            if max.2 < *n {
                max = (*id, *c, *n);
            } else if max.2 == *n {
                if *id < max.0 {
                    max = (*id, *c, *n);
                } else if *id == max.0 {
                    if *c < max.1 {
                        max = (*id, *c, *n);
                    }
                }
            }
        }
        v.push(format!("{}:{}", max.0, max.1.to_string().repeat(max.2)));
        hs.remove(&max);
    }
    v.join("/")
}

fn populate(s: &str, str_id: char, hs: &mut HashSet<(char, char, usize)>)  {
    let mut hm = HashMap::new();
    for c in s.chars().filter(|&c| c >= 'a' && c <= 'z') {
        let count = hm.entry(c).or_insert(0);
        *count += 1;
    }
    for (c, n) in hm.iter() {
        if *n <= 1 {
            continue;
        }
        if hs.iter().filter(|(_, cx, nx)| c == cx && n == nx).count() > 0 {
            hs.remove(&('1', *c, *n));
            hs.insert(('=', *c, *n));
        } else if hs.iter().filter(|(_, cx, nx)| c == cx && n > nx).count() > 0 {
            hs.retain(|(_, cx, _)| c != cx );
            hs.insert((str_id, *c, *n));
        } else if hs.iter().filter(|(_, cx, nx)| c == cx && n < nx).count() == 0 {
            hs.insert((str_id, *c, *n));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }

    #[test]
    fn basics_mix() {

        testing("Are they here", "yes, they are here", 
            "2:eeeee/2:yy/=:hh/=:rr");
        testing("looping is fun but dangerous", "less dangerous than coding", 
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
        testing(" In many languages", " there's a pair of functions", 
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing("A generation must confront the looming ", "codewarrs", 
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
        
    }
}
