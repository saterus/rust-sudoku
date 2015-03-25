use std::num::Float;

pub fn assert_square(num: usize) {
    let root = (num as f32).sqrt();
    assert!(root == root.trunc(), "{:?} must be an even square!", num);
}

pub fn square_root(num: usize) -> usize {
    assert_square(num);
    (num as f32).sqrt().trunc() as usize
}
