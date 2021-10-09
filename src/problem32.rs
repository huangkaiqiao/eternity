use log::info;
use crate::Solution;

impl Solution{

    pub fn longest_valid_parentheses(s:String) -> i32 {
        let len = s.len();
        let cs = s.chars().collect::<Vec<_>>();
        let mut plen:[i32;3*10000] = [0;30000];  // parentheses_len
        let mut max = 0;
        if len>1 && &s[0..2] == "()" {
            plen[1] = 2;
            max = 2;
        } 
        for i in 2..len{
            if &s[(i-1)..i] == "()" {
                // info!("(), {}", plen[i-2]+2);
                plen[i] = std::cmp::max(plen[i-2]+2, plen[i])
            } else if cs[i] == ')' && i as i32 >= plen[i-1]+1 && cs[i-1-(plen[i-1] as usize)] == '(' {
                let mut part1 = 0;
                if i as i32 >= plen[i-1] + 2 {
                    part1 = plen[i-2-(plen[i-1] as usize)]
                }
                let part2 = plen[i-1] + 2;
                // info!("{}, {}, {}, {}, {}", i, plen[i-1]+2, part1, plen[i-2-(plen[i-1]as usize)], i-2-(plen[i-1]as usize));
                plen[i] = std::cmp::max(part1+part2, plen[i])
            }
            max = std::cmp::max(plen[i], max);
            // info!("plen[{}]:{}", i, plen[i]);
        }
        max   
    }
}