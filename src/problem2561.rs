use crate::Solution;

impl Solution {

    // fn remove_shared_elements<T: Eq + Copy>(vec1: &mut Vec<T>, vec2: &mut Vec<T>) {
    //     let mut i = 0;
    //     while i < vec1.len() {
    //         if let Some(j) = vec2.iter().position(|x| *x == vec1[i]) {
    //             vec1.remove(i);
    //             vec2.remove(j);
    //         } else {
    //             i += 1;
    //         }
    //     }
    // }

    // pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    //     let v1 = basket1.clone();
    //     let v2 = basket2.clone();
    //     remove_shared_elements(&mut v1, &mut v2);
    //     for i in (0..v1.len()) {
    //         for j in (0..v2.len()){
    //             let v1clone = v1.clone();
    //             let v2clone = v2.clone();
    //             let tmp = v1clone[i];
    //             v1[i] = v2[j];
    //             v2[j] = tmp;
    //         }
    //     }
    // }

    /* check whether a vector is equal to another vector that contains NaN and inf? */
    // fn eq_with_nan_eq(a: f64, b: f64) -> bool {
    //     (a.is_nan() && b.is_nan()) || (a == b)
    // }

    // fn vec_compare(va: &[f64], vb: &[f64]) -> bool {
    //     (va.len() == vb.len()) &&  // zip stops at the shortest
    //     va.iter()
    //     .zip(vb)
    //     .all(|(a,b)| eq_with_nan_eq(*a,*b))
    // }
}