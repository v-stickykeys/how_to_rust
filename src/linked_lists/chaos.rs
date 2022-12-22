pub fn wrong_order() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        // let ref2 = &mut *ref1;
        // let ref3 = &mut *ref2;

        *ref1 += 1;
        *ptr2 += 2;
        // *ref2 += 2;
        // *ref3 += 3;

        // println!("ref3: {}", ref3);
        // println!("ref2: {}", ref2);
        // println!("ref1: {}", ref1);
        println!("data: {}", data);
    }
}

pub fn right_order() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        // *ptr2 += 2;

        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}

pub fn pointer_offset() {
    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0];
        let ptr2_at_0 = ref1_at_0 as *mut i32;
        let ptr3_at_1 = ptr2_at_0.add(1);

        *ptr3_at_1 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        // expect [3, 3, 0, ...]
        println!("{:?}", &data[..]);
    }
}

pub fn pointer_offset_copy() {
    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0];
        let ptr2_at_0 = ref1_at_0 as *mut i32;
        let ptr3_at_0 = ptr2_at_0;

        *ptr3_at_0 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        // expect [6, 0, 0, ...]
        println!("{:?}", &data[..]);
    }
}

pub fn mess() {
    unsafe {
        let mut data = [0; 10];
        let ref1 = &mut data[0];
        let ptr2 = ref1 as *mut i32;
        let ptr3 = ptr2;
        let ptr4 = ptr2.add(0);
        let ptr5 = ptr3.add(1).sub(1);

        *ptr3 += 3;
        *ptr2 += 2;
        *ptr4 += 4;
        *ptr5 += 5;
        *ptr3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        // expect [20, 0, 0, ...]
        println!("{:?}", &data[..]);

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wrong_order() {
        // wrong_order();
    }

    #[test]
    fn test_right_order() {
        right_order();
    }

    #[test]
    fn test_pointer_offset() {
        // pointer_offset();
    }

    #[test]
    fn test_pointer_offset_copy() {
        pointer_offset_copy();
    }

    #[test]
    fn test_mess() {
        mess();
    }
}