// https://www.codewars.com/kata/57ea70aa5500adfe8a000110/train/rust

fn fold_array(arr: &[i32], runs: usize) -> Vec<i32> {
    todo!()
}

pub fn test_fold_an_array() -> bool {
    let input = [1, 2, 3, 4, 5];
    if fold_array(&input, 1) != [6, 6, 3] { println!("1"); return false; };
    if fold_array(&input, 2) != [9, 6] { println!("2"); return false; };
    if fold_array(&input, 3) != [15] { println!("3"); return false; };
        
    let input = [-9, 9, -8, 8, 66, 23];
    if fold_array(&input, 1) != [14, 75, 0] { println!("4"); return false; };

    true
}