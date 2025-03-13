impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut variable2index = std::collections::HashMap::with_capacity(equations.len() * 2 + 1);
        let mut index = move |variable: String, allow_insert: bool| {
            let size = variable2index.len();
            if allow_insert {
                *variable2index.entry(variable).or_insert(size)
            } else {
                *variable2index.get(&variable).unwrap_or(&size)
            }
        };
        let pair = |mut pair: Vec<String>| {
            let mut iter = pair.drain(..2);
            (iter.next().unwrap(), iter.next().unwrap())
        };
        let equations = equations
            .into_iter()
            .map(|equation| {
                let (a, b) = pair(equation);
                (index(a, true), index(b, true))
            })
            .collect::<Vec<_>>();
        let size = index(String::new(), false);
        let mut matrix = vec![vec![None; size]; size];
        for (index, row) in matrix.iter_mut().enumerate() {
            row[index] = Some(1.);
        }
        for ((a, b), value) in equations.into_iter().zip(values.into_iter()) {
            matrix[a][b] = Some(value);
            matrix[b][a] = Some(1. / value);
        }
        let mut queue = std::collections::VecDeque::new();
        queries
            .into_iter()
            .map(|query| {
                let (a, b) = pair(query);
                let (a, target) = (index(a, false), index(b, false));
                if a == size || target == size {
                    return -1.;
                }
                if let Some(result) = matrix[a][target] {
                    return result;
                }
                let mut seen = vec![false; size];
                queue.clear();
                queue.push_back((a, 1.));

                while let Some((a, a_product)) = queue.pop_front() {
                    for b in 0..size {
                        if a == b || seen[b] {
                            continue;
                        }
                        if let Some(multiplier) = matrix[a][b] {
                            let b_product = a_product * multiplier;
                            if let Some(multiplier) = matrix[b][target] {
                                return multiplier * b_product;
                            } else {
                                seen[b] = true;
                                queue.push_back((b, b_product));
                            }
                        }
                    }
                }
                -1.
            })
            .collect()
    }
}