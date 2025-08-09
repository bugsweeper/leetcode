// Last updated: 09.08.2025, 21:14:38
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 1{
            return true;
        }
        if n<=0{
            return false;
        }
        for i in 0..5{
            if 2_u32.pow(i as u32) == n as u32{
                return true;
            }
        }
        for i in 5..(n.isqrt()+1){
            if 2_u32.pow(i as u32) == n as u32{
                return true;
            }
        }
        return false;
    }
}