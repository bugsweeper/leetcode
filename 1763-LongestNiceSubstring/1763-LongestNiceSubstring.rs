// Last updated: 19.06.2025, 14:39:43
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let [edge0, edge1, ..] = &edges[..] else {
            unimplemented!();
        };
        let (u0, v0, u1, v1) = (edge0[0], edge0[1], edge1[0], edge1[1]);
        if u0 == u1 || u0 == v1 {
            u0
        } else {
            v0
        }
    }
}