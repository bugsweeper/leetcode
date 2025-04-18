// Last updated: 18.04.2025, 10:25:11
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut rle = vec![1];
        for _ in 1..n {
            let capacity = rle.len() * 2;
            let mut iter = rle.into_iter();
            let mut count = 1;
            let mut digit = iter.next().unwrap();
            (rle, count, digit) = iter.fold(
                (Vec::with_capacity(capacity), count, digit),
                |(mut rle, mut count, mut digit), num| {
                    if digit == num {
                        count += 1;
                    } else {
                        rle.push(count);
                        rle.push(digit);
                        count = 1;
                        digit = num;
                    }
                    (rle, count, digit)
                },
            );
            rle.push(count);
            rle.push(digit);
        }
        String::from_iter(rle.into_iter().map(|digit| (digit + b'0') as char))
    }
}
