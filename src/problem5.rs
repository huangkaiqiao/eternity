// use log::info;
use crate::Solution;

impl Solution{
    // pub fn longest_palindrome(s:String) -> String {
    //     // info!("longest_palindrome");
    //     let len = s.len();
    //     let mut odd:[usize;1000] = [1;1000];
    //     let mut even = [0;1000];
    //     let mut max = 1;
    //     let mut start = 0;
    //     for i in 1..len {
    //         let olen = odd[i-1];
    //         let elen = even[i-1];
    //         if i-1 >= olen && s.chars().nth(i) == s.chars().nth(i-olen-1) {
    //             odd[i] = odd[i-1] + 2;
    //         } 
    //         if i-1 >= elen && s.chars().nth(i) == s.chars().nth(i-elen-1) {
    //             even[i] = even[i-1] + 2;
    //         } 
    //         if odd[i] > max {
    //             max = odd[i];
    //             start = i + 1 - max;
    //         }
    //         if even[i] > max {
    //             max = even[i];
    //             start = i + 1 - max;
    //         }
    //         info!("i={}, start={}, max={}, olen={}, elen={}, odd={}, even={}", i, start, max, olen, elen, odd[i], even[i]);
    //         // info!("char_at({})={}",i ,s.chars().nth(i).unwrap());
    //     }
    //     // info!("start={}, max={}", start, max);
    //     s[start..(start+max)].to_owned()
    // }
    
    // pub fn longest_palindrome(s:String) -> String {
    //     let len = s.len();
    //     let bytes = s.as_bytes();
    //     if len == 1 {
    //         return s;
    //     } 
    //     if len == 2 {
    //         if bytes[0] == bytes[1] {
    //             return s
    //         }
    //         return s[0..1].to_owned();
    //     }
    //     if bytes[0] == bytes[len-1] {
    //         let tmp = Solution::longest_palindrome((&s[1..(len-1)]).to_owned());
    //         if tmp.len() != len-2 {
    //             return "".to_owned();
    //         }else {
    //             return s
    //         }
    //     } else {
    //         let lp1 =  Solution::longest_palindrome((&s[0..(len-1)]).to_owned());
    //         let lp2 = Solution::longest_palindrome((&s[1..len]).to_owned());
    //         let first_char = s[0..1].to_owned();
    //         if lp1.len() >= lp2.len() {
    //             if lp1.len() > first_char.len() {
    //                 lp1
    //             } else {
    //                 first_char
    //             }
    //         }else {
    //             lp2
    //         }
    //     }
    // }

    pub fn longest_palindrome(s:String) -> String {
        let len = s.len();
        let bytes = s.as_bytes();
        let lp:&mut usize = &mut 0;
        let max:&mut usize = &mut 1;
        for i in 0..len {
            Solution::extend_palindrome(bytes, i, i, lp, max);
            Solution::extend_palindrome(bytes, i+1, i, lp, max);
        }
        s[*lp..(*lp+*max)].to_owned()
    }

    fn extend_palindrome(bytes:&[u8], mut j:usize, mut k:usize, lp:&mut usize, max:&mut usize) {
        while 0 < j && k+1 < bytes.len() && bytes[j-1] == bytes[k+1]{
            j-=1;
            k+=1;
        }
        let len = 1 + k - j;
        if len > *max {
            *max = len;
            *lp = j;
        } 
        // info!("j={}, k={}", j, k);
    }
}