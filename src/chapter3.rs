/**
 * データ型
 */
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Ok
    let spaces = "   ";
    let spaces = spaces.len();

    // Err: cannot assign a different type
    // let mut spaces = "   ";
    // let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("{}", tup.1);

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
    println!("{}", months[3]);

    another_function(five());
}

/**
 * 関数
 * - 命名規則: スネークケース（変数も）
 */
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("Plus one: {}", plus_one(2));
    control_flow();
}

// note: 最後の指揮を暗黙的にreturnする
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // doesn't return
    // x + 1;

    // returns
    x + 1
}

fn control_flow() {
    let x = 5;
    let y = 6;

    if x < y {
        println!("x is less than y");
    } else {
        println!("x is not less than y");
    }

    if x == 5 && y == 6 {
        println!("x is five and y is six");
    }

    // note: variables should be a single type
    let bool_number = if x < y { 1 } else { 0 };
    println!("bool_number: {}", bool_number);

    let range = 0..=10;
    for i in range {
        println!("i: {}", i);
    }
}
