// Last updated: 22.03.2025, 14:26:42
use std::collections::HashSet;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut vertices = vec![None; n];
        let mut graphs = Vec::with_capacity(n.div_ceil(2));
        for edge in edges {
            let [a, b] = edge[..] else {
                continue;
            };
            let (a, b) = (a as usize, b as usize);
            match (vertices[a], vertices[b]) {
                (Some(ga), Some(gb)) => {
                    if ga != gb {
                        let graph_b: &mut (i32, HashSet<usize>) = graphs.get_mut(gb).unwrap();
                        let mut source = HashSet::new();
                        std::mem::swap(&mut graph_b.1, &mut source);
                        let graph_b_edges = graph_b.0;
                        let graph_a: &mut (i32, HashSet<usize>) = graphs.get_mut(ga).unwrap();
                        for vertex_b in HashSet::drain(&mut source) {
                            vertices[vertex_b] = Some(ga);
                            HashSet::insert(&mut graph_a.1, vertex_b);
                        }
                        graph_a.0 += graph_b_edges + 1;
                    } else {
                        let graph_a: &mut (i32, HashSet<usize>) = graphs.get_mut(ga).unwrap();
                        graph_a.0 += 1;
                    }
                }
                (Some(graph_index), _) => {
                    let graph: &mut (i32, HashSet<usize>) = graphs.get_mut(graph_index).unwrap();
                    HashSet::insert(&mut graph.1, b);
                    graph.0 += 1;
                    vertices[b] = Some(graph_index);
                }
                (_, Some(graph_index)) => {
                    let graph: &mut (i32, HashSet<usize>) = graphs.get_mut(graph_index).unwrap();
                    HashSet::insert(&mut graph.1, a);
                    graph.0 += 1;
                    vertices[a] = Some(graph_index);
                }
                _ => {
                    let graph = HashSet::from([a, b]);
                    let graph_index = graphs.len();
                    graphs.push((1, graph));
                    vertices[a] = Some(graph_index);
                    vertices[b] = Some(graph_index);
                }
            }
        }
        let (graph_count, graph_vertices_sum) = graphs.into_iter().fold(
            (0, 0),
            |(graph_count, graph_vertices_sum), (graph_edges, graph)| {
                let graph_vertices = graph.len();
                (
                    graph_count
                        + if graph_vertices > 0
                            && graph_edges as usize == graph_vertices * (graph_vertices - 1) / 2
                        {
                            1
                        } else {
                            0
                        },
                    graph_vertices_sum + graph.len(),
                )
            },
        );
        (n - graph_vertices_sum + graph_count) as i32
    }
}