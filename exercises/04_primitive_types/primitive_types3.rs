use std::vec;

fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut a: Vec<u32> = vec![];

    for i in 0..100 {
        a.push(i);
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
