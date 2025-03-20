fn correct_mutation(start: &[u8], end: &[u8]) -> bool {
    let mut diff = 0;
    for (start, end) in start.iter().zip(end.iter()) {
        if *start != *end {
            diff += 1;
            if diff > 1 {
                return false;
            }
        }
    }
    diff == 1
}

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        if start_gene == end_gene {
            return 0;
        }
        if bank.iter().all(|gene| end_gene.ne(gene)) {
            return -1;
        }
        if correct_mutation(start_gene.as_bytes(), end_gene.as_bytes()) {
            return 1;
        }
        let mut used_bank = vec![false; bank.len()];
        let mut used_count = 0;
        let mut mutations = 1;
        let mut current_generation = vec![start_gene.as_str()];
        let mut next_generation = Vec::with_capacity(bank.len());
        while used_count < bank.len() {
            while let Some(gene) = current_generation.pop() {
                if correct_mutation(gene.as_bytes(), end_gene.as_bytes()) {
                    return mutations;
                }
                for (index, next_gene) in bank.iter().enumerate() {
                    if used_bank[index] {
                        continue;
                    }
                    if correct_mutation(gene.as_bytes(), next_gene.as_bytes()) {
                        next_generation.push(next_gene.as_str());
                        used_bank[index] = true;
                        used_count += 1;
                    }
                }
            }
            if next_generation.is_empty() && used_count < bank.len() {
                return -1;
            }
            mutations += 1;
            std::mem::swap(&mut current_generation, &mut next_generation);
        }
        -1
    }
}