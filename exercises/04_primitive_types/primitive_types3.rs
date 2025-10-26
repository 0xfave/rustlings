fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [1,2,3,1,2,3,5,6,3,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,7,7,8,8,8,8,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,7,8,8,8,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,4,4,4];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
