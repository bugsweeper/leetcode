fn has_cycle(course: usize, connections: &[Vec<usize>], in_stack: &mut [bool], checked: &mut [bool]) -> bool {
    in_stack[course] = true;
    let course_connections = connections.get(course).unwrap();
    for &before in course_connections {
        if checked[before] {
            continue;
        }
        if in_stack[before] || has_cycle(before, connections, in_stack, checked) {
            return true;
        }
    }
    checked[course] = true;
    in_stack[course] = false;
    false
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut connections = vec![Vec::with_capacity(num_courses); num_courses];
        for prerequisite in prerequisites {
            let [after, before] = prerequisite[..] else {
                continue;
            };
            connections[after as usize].push(before as usize);
        }
        let mut checked = vec![false; num_courses as usize];
        let mut in_stack = vec![false; num_courses as usize];
        for course in 0..num_courses {
            if !checked[course] && has_cycle(course, &connections, &mut in_stack, &mut checked) {
                return false;
            }
        }
        true
    }
}