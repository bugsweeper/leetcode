
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const PALINDROMES_AMOUNT: usize = ABC_LEN * ABC_LEN;

impl Solution {
  pub fn count_palindromic_subsequence(s: String) -> i32 {
    let mut palindromes_len = [0; PALINDROMES_AMOUNT];
    let s_bytes = s.as_bytes();
    for &c in s_bytes {
      let c = (c - b'a') as usize;
      let middle_is_same_as_edge = *unsafe{palindromes_len.get_unchecked(c*ABC_LEN+c)};
      // Count edges
      for len in &mut palindromes_len[c*ABC_LEN..(c+1)*ABC_LEN] {
        match *len {
          0 => *len = 1,
          2 => {
            *len = 3;
          },
          _ => {}
        }
      }
      // Count middles
      for i in (c..PALINDROMES_AMOUNT).step_by(ABC_LEN) {
        // Safety index i can't reach PALINDROMES_AMOUNT, which is size of palindromes_len
        let len = unsafe{palindromes_len.get_unchecked_mut(i)};
        if *len == 1 {
          *len = 2;
        }
      }
      // Do not count twice if middle equals edge
      *unsafe{palindromes_len.get_unchecked_mut(c*ABC_LEN+c)} = 3.min(middle_is_same_as_edge + 1);
    }
    palindromes_len.into_iter().filter(|&len| len == 3).count() as i32
  }
}