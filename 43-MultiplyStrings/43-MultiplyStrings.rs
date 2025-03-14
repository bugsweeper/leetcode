impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1_digits = num1.as_bytes().iter().rev().map(|digit| *digit - b'0').collect::<Vec<_>>();
        let num2_digits = num2.as_bytes().iter().rev().map(|digit| *digit - b'0').collect::<Vec<_>>();
        let mut result_digits = vec![0; num1_digits.len() + num2_digits.len()];
        for (index1, num1_digit) in num1_digits.into_iter().enumerate() {
            let mut carry = 0;
            for (index2, num2_digit) in num2_digits.iter().enumerate() {
                let result_index = index1 + index2;
                let result_digit = result_digits[result_index] + carry + num1_digit * num2_digit;
                result_digits[result_index] = result_digit % 10;
                carry = result_digit / 10;
            }
            if carry > 0 {
                let index = index1 + num2_digits.len();
                while carry > 0 {
                    let result_digit = result_digits[index] + carry;
                    result_digits[index] = result_digit % 10;
                    carry = result_digit / 10;
                }
            }
        }
        let mut zero_index = result_digits.len() - 1;
        while zero_index > 0 && result_digits[zero_index] == 0 {
            zero_index -= 1;
        }
        result_digits.truncate(zero_index + 1);
        let end = result_digits.len() - 1;
        for i in 0..result_digits.len() / 2 {
            result_digits[i] += b'0';
            result_digits.swap(i, end - i);
            result_digits[i] += b'0';
        }
        if result_digits.len() % 2 == 1 {
            let index = result_digits.len() / 2;
            result_digits[index] += b'0';
        }
        String::from_utf8(result_digits).unwrap()
    }
}