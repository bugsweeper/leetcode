#[derive(Clone)]
struct BitSet {
    data: Vec<u128>,
}

const CHUNK_BYTES: usize = (u128::BITS / u8::BITS) as usize;

impl BitSet {
    fn new(first_value: usize, total_size: usize) -> Self {
        let mut data = vec![0; total_size.div_ceil(CHUNK_BYTES)];
        *unsafe{data.get_unchecked_mut(first_value / CHUNK_BYTES)} = 1 << (first_value % CHUNK_BYTES);
        Self{
            data,
        }
    }
    fn contains(&self, item_index: usize) -> bool {
        *unsafe{self.data.get_unchecked(item_index / CHUNK_BYTES)} & (1 << (item_index % CHUNK_BYTES)) != 0
    }
    fn merge(&mut self, other: &BitSet) {
        for (chunk_mut, chunk) in self.data.iter_mut().zip(other.data.iter()) {
            *chunk_mut |= *chunk;
        }
    }
    fn assign(&mut self, other: &BitSet) {
        self.data.clone_from_slice(&other.data);
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut connections = (0..n).map(|node| BitSet::new(node, n)).collect::<Vec<_>>();

        for edge in edges {
            let from = *unsafe{edge.get_unchecked(0)} as usize - 1;
            let to = *unsafe{edge.get_unchecked(1)} as usize - 1;
            let from_connections = unsafe{connections.get_unchecked(from)};
            let to_connections = unsafe{connections.get_unchecked(to)};
            if from_connections.contains(to) || to_connections.contains(from) {
                return edge;
            }
            let mut united = from_connections.clone();
            united.merge(to_connections);
            for i in 0..n {
                if united.contains(i) {
                    unsafe{connections.get_unchecked_mut(i)}.assign(&united);
                }
            }
        }
        unimplemented!()
    }
}