pub fn main() {
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

#[cfg(test)]
mod test {
    use crate::linked_lists::chaos::main;

    #[test]
    fn basics() {
        main()
    }
}