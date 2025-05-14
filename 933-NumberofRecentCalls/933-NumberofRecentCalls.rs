// Last updated: 14.05.2025, 23:13:21
impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num = num;
        let mut carry = 0;
        let mut k = k;
        for digit in num.iter_mut().rev() {
            let sum = k % 10 + carry + *digit;
            *digit = sum % 10;
            carry = sum / 10;
            k /= 10;
            if k + carry == 0 {
                break;
            }
        }
        while k + carry > 0 {
            let sum = k % 10 + carry;
            carry = sum / 10;
            k /= 10;
            num.insert(0, sum % 10);
        }
        num
    }
}