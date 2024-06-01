use crate::libc;

pub fn rand() -> i32 {
    unsafe { libc::rand() }
}

pub fn set_seed(seed: u32) {
    unsafe { libc::srand(seed) }
}
