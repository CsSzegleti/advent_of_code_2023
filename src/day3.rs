use std::{ops::Range, vec};

pub struct Engine {
    schematic: Vec<String>
}

impl Engine {
    pub fn from(raw_schematic: &str) -> Engine {
        let mut schematic: Vec<String> = vec![];

        for line in raw_schematic.lines() {
            schematic.push(String::from(line.trim()));
        }


        Engine {
            schematic
        }
    }

    fn is_numeric(&self, row: usize, col: usize) -> bool {
        self.schematic[row]
            .chars().nth(col).unwrap()
                .is_numeric()
    }

    fn is_special(&self, row: usize, col: usize) -> bool {
        let char = self.schematic[row]
            .chars().nth(col).unwrap();

        !char.is_numeric() && !char.eq_ignore_ascii_case(&'.')
    }

    fn get_number(&self, row: usize, col: usize) -> (u32, Range<usize>) {
        let mut start_idx: usize = col;
        let mut end_idx: usize = col;

        loop {
            if !start_idx.overflowing_sub(1).1 && self.is_numeric(row, start_idx - 1) {
                start_idx -= 1;
            } else {
                break;
            }
        }

        loop {
            if end_idx + 1 < self.schematic.len() && self.is_numeric(row, end_idx + 1) {
                end_idx += 1;
            } else {
                break;
            }
        }

        (String::from(&self.schematic[row][start_idx..end_idx + 1]).parse::<u32>().unwrap(),
        (start_idx..end_idx + 1))
    }

    fn is_row_out_of_bounds(&self, row: usize, i:isize) -> bool {
        row.overflowing_add_signed(i).1
            || row.overflowing_add_signed(i).0 >= self.schematic.len()
    }

    fn is_col_out_of_bounds(&self, row: usize, col: usize, i:isize) -> bool {
        col.overflowing_add_signed(i).1
            || col.overflowing_add_signed(i).0 >= self.schematic[row].len()
    }

    fn sum_surroundings(&self, row: usize, col: usize) -> u32 {
        if !self.is_special(row, col) {
            return 0;
        }

        let mut sum = 0;
        
        for i in -1..=1 {
            if self.is_row_out_of_bounds(row, i) {
                continue;
            }

            let mut ranges:Vec<Range<usize>> = Vec::new();
            
            for j in -1..=1 {
                
                let tmp_row = row.overflowing_add_signed(i).0;
                let tmp_col = col.overflowing_add_signed(j).0;
                
                if self.is_col_out_of_bounds(row, col, j)
                    || self.is_processed(ranges.clone(), &tmp_col) {
                    continue;
                }

                
                if self.is_numeric(tmp_row, tmp_col) {
                    let number = self.get_number(tmp_row, tmp_col);
                    sum += number.0;
                    ranges.push(number.1);
                }
            }
        }

        sum
    }

    fn is_processed(&self, processed: Vec<Range<usize>>, idx: &usize) -> bool {
        for range in processed {
            if range.contains(idx) {
                return true
            }
        }
        false
    }


    pub fn get_engine_num(&self) -> u32 {
        let mut engine_num: u32 = 0;
        for (row_num, row) in self.schematic.iter().enumerate() {
            for (col_num, _) in row.chars().enumerate() {
                engine_num += self.sum_surroundings(row_num, col_num);
            }
        }

        engine_num
    }

    
}