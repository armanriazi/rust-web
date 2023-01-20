pub mod core;
pub mod models;
pub struct A {
    value: u8,
}

impl A {
    pub fn new(value: u8) -> A {
        A { value }
    }

    pub fn get_value(&self) -> u8 {
        let result = core::sys_a::do_stuff_from_system_a();
        self.value + result
    }
}
