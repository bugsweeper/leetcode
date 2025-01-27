use std::collections::HashSet;

fn dfs(course: usize, prerequisites: &Vec<HashSet<usize>>, indirect: &mut Vec<u128>) -> u128 {
    let course_state = *unsafe{indirect.get_unchecked(course)};
    if course_state == 0 {
        let mut indirect_prerequisites = 1 << course;
        for &prerequisite in unsafe{prerequisites.get_unchecked(course)} {
            let prerequisite_state = *unsafe{indirect.get_unchecked(prerequisite)};
            indirect_prerequisites |= if prerequisite_state == 0 { dfs(prerequisite, prerequisites, indirect) } else { prerequisite_state };
        }
        *unsafe{indirect.get_unchecked_mut(course)} = indirect_prerequisites;
        indirect_prerequisites
    } else {
        course_state
    }
}

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let num_courses = num_courses as usize;
        let mut prerequisites_set = vec![HashSet::new();num_courses];
        for prerequisite in &prerequisites {
            let [prerequisite, course] = prerequisite[..] else { continue };
            unsafe{prerequisites_set.get_unchecked_mut(course as usize)}.insert(prerequisite as usize);
        }

        let mut indirect = vec![0; num_courses];
        for course in 0..num_courses {
            let _ = dfs(course, &prerequisites_set, &mut indirect);
        }

        queries.into_iter().map(|query| {
            let [prerequisite, course] = query[..] else { return false };
            *unsafe{indirect.get_unchecked(course as usize)} & 1 << prerequisite != 0
        }).collect()
    }
}