// Last updated: 18.05.2025, 15:20:21
use std::collections::HashMap;

const MODULO: i32 = 1_000_000_007;

fn get_count1(
    step: usize,
    limit: usize,
    prev1: i32,
    cache: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    if let Some(&count) = cache.get(&(prev1, step)) {
        return count;
    }
    let mut count = 0;
    for cur1 in 0..3 {
        if cur1 == prev1 {
            continue;
        }
        count = if step == limit {
            count + 1
        } else {
            (count + get_count1(step + 1, limit, cur1, cache)) % MODULO
        };
    }
    cache.insert((prev1, step), count);
    count
}

fn get_count2(
    step: usize,
    limit: usize,
    prev1: i32,
    prev2: i32,
    cache: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    let prev = (prev1 << 2) + prev2;
    if let Some(&count) = cache.get(&(prev, step)) {
        return count;
    }
    let mut count = 0;
    for cur1 in 0..3 {
        if cur1 == prev1 {
            continue;
        }
        for cur2 in 0..3 {
            if cur2 == prev2 || cur2 == cur1 {
                continue;
            }
            count = if step == limit {
                count + 1
            } else {
                (count + get_count2(step + 1, limit, cur1, cur2, cache)) % MODULO
            };
        }
    }
    cache.insert((prev, step), count);
    count
}

fn get_count3(
    step: usize,
    limit: usize,
    prev1: i32,
    prev2: i32,
    prev3: i32,
    cache: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    let prev = (prev1 << 4) + (prev2 << 2) + prev3;
    if let Some(&count) = cache.get(&(prev, step)) {
        return count;
    }
    let mut count = 0;
    for cur1 in 0..3 {
        if cur1 == prev1 {
            continue;
        }
        for cur2 in 0..3 {
            if cur2 == prev2 || cur2 == cur1 {
                continue;
            }
            for cur3 in 0..3 {
                if cur3 == prev3 || cur3 == cur2 {
                    continue;
                }
                count = if step == limit {
                    count + 1
                } else {
                    (count + get_count3(step + 1, limit, cur1, cur2, cur3, cache)) % MODULO
                };
            }
        }
    }
    cache.insert((prev, step), count);
    count
}

fn get_count4(
    step: usize,
    limit: usize,
    prev1: i32,
    prev2: i32,
    prev3: i32,
    prev4: i32,
    cache: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    let prev = (prev1 << 6) + (prev2 << 4) + (prev3 << 2) + prev4;
    if let Some(&count) = cache.get(&(prev, step)) {
        return count;
    }
    let mut count = 0;
    for cur1 in 0..3 {
        if cur1 == prev1 {
            continue;
        }
        for cur2 in 0..3 {
            if cur2 == prev2 || cur2 == cur1 {
                continue;
            }
            for cur3 in 0..3 {
                if cur3 == prev3 || cur3 == cur2 {
                    continue;
                }
                for cur4 in 0..3 {
                    if cur4 == prev4 || cur4 == cur3 {
                        continue;
                    }
                    count = if step == limit {
                        count + 1
                    } else {
                        (count + get_count4(step + 1, limit, cur1, cur2, cur3, cur4, cache))
                            % MODULO
                    };
                }
            }
        }
    }
    cache.insert((prev, step), count);
    count
}

fn get_count5(
    step: usize,
    limit: usize,
    prev1: i32,
    prev2: i32,
    prev3: i32,
    prev4: i32,
    prev5: i32,
    cache: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    let prev = (prev1 << 8) + (prev2 << 6) + (prev3 << 4) + (prev4 << 2) + prev5;
    if let Some(&count) = cache.get(&(prev, step)) {
        return count;
    }
    let mut count = 0;
    for cur1 in 0..3 {
        if cur1 == prev1 {
            continue;
        }
        for cur2 in 0..3 {
            if cur2 == prev2 || cur2 == cur1 {
                continue;
            }
            for cur3 in 0..3 {
                if cur3 == prev3 || cur3 == cur2 {
                    continue;
                }
                for cur4 in 0..3 {
                    if cur4 == prev4 || cur4 == cur3 {
                        continue;
                    }
                    for cur5 in 0..3 {
                        if cur5 == prev5 || cur5 == cur4 {
                            continue;
                        }
                        count = if step == limit {
                            count + 1
                        } else {
                            (count
                                + get_count5(step + 1, limit, cur1, cur2, cur3, cur4, cur5, cache))
                                % MODULO
                        };
                    }
                }
            }
        }
    }
    cache.insert((prev, step), count);
    count
}

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut cache = HashMap::with_capacity(n * 3 * 2usize.pow(m as u32 - 1));
        let limit = n - 1;
        match m {
            1 => get_count1(0, limit, 3, &mut cache),
            2 => get_count2(0, limit, 3, 3, &mut cache),
            3 => get_count3(0, limit, 3, 3, 3, &mut cache),
            4 => get_count4(0, limit, 3, 3, 3, 3, &mut cache),
            5 => get_count5(0, limit, 3, 3, 3, 3, 3, &mut cache),
            _ => unimplemented!(),
        }
    }
}