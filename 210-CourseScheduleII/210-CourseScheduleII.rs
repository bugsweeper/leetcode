fn check_cycle_and_rank(
    course: usize,
    connections: &[Vec<usize>],
    in_stack: &mut [bool],
    checked: &mut [bool],
    rank: &mut [(usize, usize)],
) -> bool {
    in_stack[course] = true;
    let course_connections = connections.get(course).unwrap();
    for &before in course_connections {
        if checked[before] {
            rank[course].0 += rank[before].0;
            continue;
        }
        if in_stack[before] || check_cycle_and_rank(before, connections, in_stack, checked, rank) {
            return true;
        }
        rank[course].0 += rank[before].0;
    }
    checked[course] = true;
    in_stack[course] = false;
    false
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut connections = vec![Vec::with_capacity(num_courses); num_courses];
        for prerequisite in prerequisites {
            let [after, before] = prerequisite[..] else {
                continue;
            };
            connections[after as usize].push(before as usize);
        }
        let mut checked = vec![false; num_courses];
        let mut in_stack = vec![false; num_courses];
        let mut rank = (0..num_courses)
            .map(|course| (1, course))
            .collect::<Vec<_>>();
        for course in 0..num_courses {
            if !checked[course]
                && check_cycle_and_rank(
                    course,
                    &connections,
                    &mut in_stack,
                    &mut checked,
                    &mut rank,
                )
            {
                return Vec::new();
            }
        }
        rank.sort_unstable_by_key(|(rank, _)| *rank);
        rank.into_iter().map(|(_, course)| course as i32).collect()
    }
}