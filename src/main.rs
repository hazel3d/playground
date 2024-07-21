fn main() {
    let num:i32 = add(3,4);

    if num >= 10 {
        println!("We big c:");
    }
    else {
        println!("We small :c");
    }
}

fn add(x:i32, y:i32) -> i32 {
    return x + y;
}