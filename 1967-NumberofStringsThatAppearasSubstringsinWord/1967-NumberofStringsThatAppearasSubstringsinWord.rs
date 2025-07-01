// Last updated: 01.07.2025, 13:33:45
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let (source, destination, n) = (source as usize, destination as usize, n as usize);
        let mut connected = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            connected[u].push(v);
            connected[v].push(u);
        }
        let mut seen = vec![false; n];
        let mut stack = Vec::with_capacity(n);
        seen[source] = true;
        stack.push(source);
        while let Some(vertex) = stack.pop() {
            if vertex == destination {
                return true;
            }
            for &neighbor in &connected[vertex] {
                let seen = &mut seen[neighbor];
                if *seen {
                    continue;
                }
                *seen = true;
                stack.push(neighbor);
            }
        }
        false
    }
}