pub fn __main__() {
    println!("Hello, world!");
    println!("{}", add(1, 2)); // -> 3
    println!("{}", sub(1, 2)); // -> -1
    println!("{}", mul(1, 2)); // -> 2
    println!("{}", div(1, 2)); // -> 0.5
    println!("{}", avg(1, 2)); // ->
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn sub(x: i32, y: i32) -> i32 {
    return x - y;
}

fn mul(x: i32, y: i32) -> i32 {
    return x * y;
}

fn div(x: i32, y: i32) -> f32 {
    return (x / y) as f32;
}

fn avg(x: i32, y: i32) -> f32 {
    return ((x + y) / 2) as f32;
}

#[test]
fn it_works() {
    // FIXME: test failed
    assert_eq!(div(1, 2), 0.5);
    assert_eq!(div(1, 2), 0.5);
}
