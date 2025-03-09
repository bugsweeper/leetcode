impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut vec = Vec::new();
        vec.resize( nums.len(), 0 );
        for el in nums {
            vec[(el as usize)-1] += 1;
        }
        let mut ret = Vec::new();
        for i in 0..vec.len(){
            if ( vec[i] == 0 ) {
                ret.push( (i + 1) as i32 );
            }
        }
        return ret;
    }
}