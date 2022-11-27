use crate::interface_sample::InterfaceSample;

pub struct Sample {
    pub size: i32,
    pub pointer: Box<i32>,
}

impl InterfaceSample for Sample {
    fn void() -> () {
        println!("void");
    }
    fn intMethod(&self, code: i32) -> i32 {
        println!("intMethod");
        return self.size;
    }
    fn reference(&self) -> i32 {
        println!("reference");
        return self.size;
    }
    fn pointer(code: i32) -> Box<i32> {
        println!("pointer");
        let ptr: Box<i32> = Box::new(code);
        return ptr;
    }
}
