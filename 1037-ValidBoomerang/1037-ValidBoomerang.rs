// Last updated: 19.05.2025, 10:32:13
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let [ref a, ref b, ref c] = points[..] else {
            unimplemented!();
        };
        let (&[ax, ay], &[bx, by], &[cx, cy]) = (&a[..], &b[..], &c[..]) else {
            unimplemented!()
        };
        let (abx, aby) = (bx - ax, by - ay);
        let (acx, acy) = (cx - ax, cy - ay);
        let vector_product_height = abx * acy - acx * aby;
        vector_product_height != 0
    }
}