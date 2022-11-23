use crate::Solution;
use log::info;

impl Solution {
  // pub fn is_match(s:String, p:String) -> bool {
  //   if p.len() == 0 {
  //     return s.len() == 0;
  //   }

  //   if p.len() == 1 {
  //     if s == p {
  //       return true;
  //     }
  //     if p == "?" {
  //       return s.len() == 1;
  //     }
  //     return p == "*";
  //   }

  //   let first = p.chars().nth(0).unwrap();
  //   info!("is_match: s={}, p={}", s, p);
  //   if first == '?' {
  //     if s.len() > 0 {
  //       return Solution::is_match(s[1..].to_owned(), p[1..].to_owned());
  //     }
  //     return false;
  //   } else if first == '*' {
  //     let mut j = 0;
  //     for i in p.chars() {
  //       if i == '*' {
  //         j+=1;
  //       } else {
  //         break;
  //       }
  //     }
  //     let pattern = if j < p.len() {
  //       p[j..].to_owned()
  //     } else {
  //       "".to_owned()
  //     };
  //     for i in 0..s.len()+1 {
  //       // let res = Solution::is_match(s[i..].to_owned(), p[1..].to_owned());
  //       let res = Solution::is_match(s[i..].to_owned(), pattern.clone());
  //       if res {
  //         // info!("is_match: s={}, p={}", s[i..].to_owned(), pattern.clone());
  //         return res;
  //       }
  //     }
  //     return false;
  //   } else {
  //     if s.len()==0 || s.chars().nth(0).unwrap() != first {
  //       return false;
  //     }
  //     let mut pr = p.clone();
  //     pr.retain(|c| c!='*');
  //     if s.len() < pr.len() {
  //       return false
  //     } 
  //     let mut i: usize = 1;
  //     while i<s.len() && i<p.len() && (p[i..i+1] == *"?" || s[i..i+1] == p[i..i+1]) {
  //       i += 1;
  //     }
  //     return Solution::is_match(s[i..].to_owned(), p[i..].to_owned());
  //   }
  // }

  pub fn is_match(s: String, p: String) -> bool {
    let mut matrix = vec![vec![false; s.len()+1]; p.len()+1];
    let s_c = s.as_bytes();
    let p_c = p.as_bytes();
    matrix[0][0] = true;
    if p.len()>0 && p_c[0] == b'*' {
      for i in 0..s.len()+1 {
        matrix[0][i] = true;
      }
    }
    for i in 0..p.len() {
      for j in 0..s.len()+1 {
        if j<s.len() && (p_c[i] == b'?' || p_c[i] == s_c[j]) {
          if matrix[i][j] == true {
            matrix[i+1][j+1] = true;
          }
        }
        if p_c[i] == b'*' {
          if matrix[i][j] {
            for k in j..s.len()+1 {
              matrix[i+1][k] = true;
            }
          }
        }
      }
    }
    // info!("is_match: matrix={:?}", matrix);
    return matrix[p.len()][s.len()];
  }
}