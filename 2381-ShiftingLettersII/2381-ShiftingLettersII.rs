const ABC_LEN: i32 = (b'z' - b'a' + 1) as i32;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut prefix_sum = vec![0; s.len() + 1];
        for shift in shifts {
            let &[left, right, direction] = &shift[..] else {
                return s;
            };
            unsafe {
                match direction {
                    0 => {
                        *prefix_sum.get_unchecked_mut(left as usize) += 1;
                        *prefix_sum.get_unchecked_mut(right as usize + 1) -= 1;
                    }
                    1 => {
                        *prefix_sum.get_unchecked_mut(left as usize) -= 1;
                        *prefix_sum.get_unchecked_mut(right as usize + 1) += 1;
                    }
                    _ => {}
                }
            }
        }
        let mut s = s;
        let mut s_bytes = unsafe { s.as_bytes_mut() };
        let mut shift = 0;
        for i in (0..s_bytes.len()).rev() {
            shift += prefix_sum[i + 1];
            let c = unsafe { s_bytes.get_unchecked_mut(i) };
            *c = ((*c - b'a') as i32 + shift).rem_euclid(ABC_LEN) as u8 + b'a';
        }
        s
    }
}