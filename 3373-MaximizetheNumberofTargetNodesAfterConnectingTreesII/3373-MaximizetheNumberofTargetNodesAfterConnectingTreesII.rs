// Last updated: 29.05.2025, 16:14:35
#[inline(always)]
fn edges2connections(edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut connections = vec![vec![]; edges.len() + 1];
    for edge in edges {
        let (u, v) = (edge[0] as usize, edge[1] as usize);
        connections[u].push(v);
        connections[v].push(u);
    }
    connections
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, m) = (edges1.len() + 1, edges2.len() + 1);
        let (connections1, connections2) = (edges2connections(edges1), edges2connections(edges2));
        let mut queue = Vec::with_capacity(m);
        queue.push((0, true));
        let mut odd_count2 = 1;
        let mut seen = vec![false; m];
        seen[0] = true;
        while let Some((index, even)) = queue.pop() {
            let connected = connections2.get(index).unwrap();
            if even {
                odd_count2 += connected.len() - 1; // one of connected is visited node
            }
            for &connection in connected {
                let seen = seen.get_mut(connection).unwrap();
                if *seen {
                    continue;
                }
                queue.push((connection, !even));
                *seen = true;
            }
        }
        odd_count2 = odd_count2.max(m - odd_count2);
        let mut state = vec![None; n];
        queue.push((0, true));
        state[0] = Some(true);
        let mut even_count1 = 1; // zero will not be counted later
        while let Some((index, even)) = queue.pop() {
            let connected = connections1.get(index).unwrap();
            if !even {
                even_count1 += connected.len() - 1; // one of connected is visited node
            }
            for &connection in connected {
                let state = &mut state[connection];
                if state.is_some() {
                    continue;
                }
                queue.push((connection, !even));
                *state = Some(!even);
            }
        }
        let odd_count1 = n - even_count1 + odd_count2;
        even_count1 += odd_count2;
        let (odd_count, even_count) = (odd_count1 as i32, even_count1 as i32);
        state
            .into_iter()
            .map(|is_even| {
                if is_even.unwrap() {
                    even_count
                } else {
                    odd_count
                }
            })
            .collect()
    }
}