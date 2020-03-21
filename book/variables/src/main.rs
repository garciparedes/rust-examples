const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let xx = 2.0;
    println!("The value of xx is: {}", xx);

    let yy: f32 = 3.0;
    println!("The value of yy is: {}", yy);

    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    let product = 4 * 30;
    println!("The value of product is: {}", product);

    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);

    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let t = true;
    println!("The value of t is: {}", t);

    let f: bool = false;
    println!("The value of f is: {}", f);

    let c = 'z';
    println!("The value of c is: {}", c);

    let z = 'ℤ';
    println!("The value of z is: {}", z);

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x_tup, y_tup, z_tup) = tup;
    println!("The value of x_tup is: {}", x_tup);
    println!("The value of y_tup is: {}", y_tup);
    println!("The value of z_tup is: {}", z_tup);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);
    let six_point_four = tup.1;
    println!("The value of six_point_four is: {}", six_point_four);
    let one = tup.2;
    println!("The value of one is: {}", one);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for month in &months {
        println!("The value is: {}", month);
    }

    let a: [i32; 5]  = [1, 2, 3, 4, 5];
    for i in &a {
        println!("The value is: {}", i);
    }

    let first = a[0];
    println!("The value of first is: {}", first);
    let second = a[1];
    println!("The value of second is: {}", second);

    let a = [3; 5];
    for i in &a {
        println!("The value is: {}", i);
    }
}
