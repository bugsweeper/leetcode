/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut size = n;
        let mut base = 1;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            match guess(middle) {
                1 => base = middle,
                0 => return middle,
                _ => {}
            }
            size -= half;
        }
        base
    }
}