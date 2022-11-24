use crate::{sum_of_digits::test_digital_root, fold_an_array::test_fold_an_array};

mod sum_of_digits;
mod fold_an_array;

fn main() {
    println!("test_digital_root: {}", test_digital_root());
    println!("fold_an_array: {}", test_fold_an_array());
}
