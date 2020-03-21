fn main() {
    // hello, world
    println!("Hello, world!");

    let x = five();
    let y = plus_one(x);
    another_function(x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}