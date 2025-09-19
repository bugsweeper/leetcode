// Last updated: 19.09.2025, 22:57:58
const ABC_LEN: usize = (b'Z' - b'A' + 1) as usize;

struct Spreadsheet {
    data: Vec<i32>
}

#[inline]
fn cell2index(cell: &str) -> usize {
    (cell[1..].parse::<usize>().unwrap() - 1) * ABC_LEN + (cell.as_bytes()[0] - b'A') as usize
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(rows: i32) -> Self {
        Self {
            data: vec![0; rows as usize * ABC_LEN]
        }
    }
    
    fn set_cell(&mut self, cell: String, value: i32) {
        self.data[cell2index(&cell)] = value;
    }
    
    fn reset_cell(&mut self, cell: String) {
        self.data[cell2index(&cell)] = 0;
    }
    
    fn get_value(&self, formula: String) -> i32 {
        let plus_position = formula.as_bytes().iter().position(|&byte| byte == b'+').unwrap();
        let first = &formula[1..plus_position];
        let first = if first.as_bytes()[0].is_ascii_digit() {
            first.parse::<i32>().unwrap()
        } else {
            self.data[cell2index(first)]
        };
        let second = &formula[plus_position + 1..];
        let second = if second.as_bytes()[0].is_ascii_digit() {
            second.parse::<i32>().unwrap()
        } else {
            self.data[cell2index(second)]
        };
        first + second
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */