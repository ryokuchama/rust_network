pub trait InterfaceSample {
    fn void() -> ();
    fn intMethod(&self, code: i32) -> i32;
    fn reference(&self) -> i32;
    fn pointer(code: i32) -> Box<i32>;
}
