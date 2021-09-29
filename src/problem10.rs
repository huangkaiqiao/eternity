use log::info;

use crate::Solution;

impl Solution {
    // pub fn is_match(s: String, p: String) -> bool {
    //     info!("s={}, p={}", s, p);
    //     let slen = s.len();
    //     let plen = p.len();
    //     if slen + plen == 0 {
    //         return true;
    //     }
    //     let chars: Vec<char> = p.chars().collect::<Vec<_>>();
    //     for i in 0..plen {
    //         if chars[i].is_alphabetic() || chars[i] == '.' {
    //             if i+1 < plen {
    //                 if chars[i+1].is_alphabetic() || chars[i+1] == '.' {
    //                     if s.chars().nth(0).unwrap() == chars[i] || chars[i] == '.' {
    //                         return Solution::is_match(s[1..slen].to_owned(), p[1..plen].to_owned());
    //                     } else {
    //                         return false;
    //                     }
    //                 } else if chars[i+1] == '*' {
    //                     let mut matches = 0;
    //                     let schars = s.chars().collect::<Vec<_>>();
    //                     while matches<slen && (schars[matches]==chars[i] || chars[i] == '.') {
    //                         matches += 1;
    //                     }
    //                     for j in (matches+2)..0 {
    //                         if Solution::is_match(s[(j-1)..slen].to_owned(), p[2..plen].to_owned()) {
    //                             return true;
    //                         }
    //                     }
    //                 } 
    //             } else {
    //                 return (chars[i] == s.chars().nth(0).unwrap() || chars[i] == '.') && Solution::is_match(s[1..slen].to_owned(), p[1..plen].to_owned());
    //             }
    //         } 
    //     }
    //     info!("done!");
    //     false
    // }

    pub fn is_match(s: String, p: String) -> bool {
        let slen = s.len();
        let plen = p.len();
        // info!("s={}, p={}", s, p);
        if plen == 0 {
            return slen == 0;
        }
        let scs = s.chars().collect::<Vec<_>>();
        let pcs = p.chars().collect::<Vec<_>>();
        if pcs[plen-1].is_alphabetic() {
            if slen == 0 {
                return false
            }
            if scs[slen-1] != pcs[plen-1] {
                return false;
            } else {
                return Solution::is_match(s[0..(slen-1)].to_owned(), p[0..(plen-1)].to_owned());
            }
        } else if pcs[plen-1] == '.' {
            if slen == 0 {
                return false
            }
            return Solution::is_match(s[0..(slen-1)].to_owned(), p[0..(plen-1)].to_owned());
        } else if pcs[plen-1] == '*' {
            if plen < 2 {
                return false
            }
            let mut matches = 0;
            if pcs[plen-2].is_alphabetic() {
                if slen == 0 {
                    return Solution::is_match(s.to_owned(), p[0..(plen-2)].to_owned())
                }
                while matches<=slen-1 && scs[slen-1-matches] == pcs[plen-2] {
                    matches += 1;
                }
                // info!("matches={}", matches);
                for i in (0..(matches+1)).rev() {
                    // info!("i={}", i);
                    if Solution::is_match(s[0..(slen-i)].to_owned(), p[0..(plen-2)].to_owned()) {
                        return true;
                    }
                }
            } else if pcs[plen-2] == '.' {
                if slen == 0 {
                    return Solution::is_match(s.to_owned(), p[0..(plen-2)].to_owned())
                }
                while matches<=slen-1 {
                    matches += 1;
                }
                for i in (0..(matches+1)).rev() {
                    if Solution::is_match(s[0..(slen-i)].to_owned(), p[0..(plen-2)].to_owned()) {
                        return true;
                    }
                }
            }
        }
        false
    }
}