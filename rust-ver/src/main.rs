#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::str_to_string)]
#![allow(clippy::cast_sign_loss)]

fn change_index(index: &mut Option<usize>, array: &[Option<i32>]) {
    *index = index
        .and_then(|x| {
            if x > array.len() {
                return None;
            };
            array[x]
        })
        .map(|x| x as usize);
}

fn has_cycle(array: &[Option<i32>]) -> bool {
    let mut slow: Option<usize> = Some(0);
    let mut fast: Option<usize> = Some(0);

    while fast.is_some() {
        change_index(&mut slow, array);
        change_index(&mut fast, array);
        change_index(&mut fast, array);

        if slow == fast {
            return true;
        }
    }
    false
}

fn main() {
    let array = [Some(2), Some(0), Some(4), Some(3), Some(1), Some(2)];
    println!("\n{:?}", array);
    println!("Has a cycle? {}", has_cycle(&array));
    let array = [Some(1), Some(-1), Some(1), Some(1), None];
    println!("\n{:?}", array);
    println!("Has a cycle? {}", has_cycle(&array));

    let array = [Some(1), Some(2), Some(3), Some(4), Some(5), None];

    println!("\n{:?}", array);
    println!("Has a cycle? {}", has_cycle(&array));

    println!("\nPlease enter a list of integers separated by spaces:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut array: Vec<Option<i32>> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().ok())
        .collect();
    array.push(None);
    println!("{:?}", array);
    println!("Has a cycle? {}", has_cycle(&array));
}
