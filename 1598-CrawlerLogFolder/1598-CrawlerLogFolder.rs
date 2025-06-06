// Last updated: 06.06.2025, 15:28:26
struct ParkingSystem {
    slots: [i32; 4]
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self{
            slots: [0, big, medium, small]
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        let slots = &mut self.slots[car_type as usize];
        if *slots == 0 {
            return false;
        }
        *slots -= 1;
        true
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */