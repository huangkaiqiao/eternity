/**
 * Alice and Bob take turns playing a game, with Alice starting first.
 * Initially, there is a number N on the chalkboard.  On each player's turn, that player makes a move consisting of:
 * 
 * Choosing any x with 0 < x < N and N % x == 0.
 * Replacing the number N on the chalkboard with N - x.
 * Also, if a player cannot make a move, they lose the game.
 * 
 * Return True if and only if Alice wins the game, assuming both players play optimally.
 * 
 * Example 1:
 *  Input: 2
 *  Output: true
 *  Explanation: Alice chooses 1, and Bob has no more moves.
 * 
 * Example 2:
 *  Input: 3
 *  Output: false
 *  Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.
 */
use crate::Solution;

// pub struct Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        if n == 1 {
            return false
        }
        let halfn: i32 = n / 2;
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        for i in 2..halfn {
            if n % i == 0 {
                v.push(i);
            } 
        }
        let mut win = vec![false; (n+1) as usize];
        let mut lose = vec![false; (n+1) as usize];
        win[2] = true;
        for i in 2..(n+1) {
            for (_pos, e) in v.iter().enumerate() {
                // let j: i32 =  as i32;
                let target = (i+e) as usize;
                if i+e <= n {
                    win[target] = lose[i as usize] || win[target];
                    lose[target] = win[i as usize] || lose[target];
                }
            }
        }
        win[n as usize]
    }
}