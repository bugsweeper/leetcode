// Last updated: 16.06.2025, 14:32:31
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types
            .into_iter().map(|box_type| (box_type[0], box_type[1]))
            .collect::<Vec<_>>();
        box_types.sort_unstable_by_key(|(_, units)| -units);
        let mut remaining_boxes = truck_size;
        let mut max_units = 0;
        for (mut boxes, units) in box_types {
            if boxes > remaining_boxes {
                boxes = remaining_boxes;
            }
            max_units += boxes * units;
            remaining_boxes -= boxes;
            if remaining_boxes == 0 {
                break;
            }
        }
        max_units
    }
}