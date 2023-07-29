//! Chapter 1
//!
//! 事始め
//!
//! ファイル自体に対してのコメントは
//! このように //! で記述する

pub fn __main__() {
    println!("Hello, world!");
    println!("{}", add(1.0, 2.0)); // -> 3
    println!("{}", sub(1.0, 2.0)); // -> -1
    println!("{}", mul(1.0, 2.0)); // -> 2
    println!("{}", div(1.0, 2.0)); // -> 0.5
    println!("{}", avg(1.0, 2.0)); // -> 1.5
}

/// 足し算
///
/// # 引数
/// * `x` - 足される数
/// * `y` - 足す数
fn add(x: f32, y: f32) -> f32 {
    return x + y;
}

fn sub(x: f32, y: f32) -> f32 {
    return x - y;
}

fn mul(x: f32, y: f32) -> f32 {
    return x * y;
}

fn div(x: f32, y: f32) -> f32 {
    return (x / y) as f32;
}

fn avg(x: f32, y: f32) -> f32 {
    return ((x + y) / 2.0) as f32;
}

#[test]
fn test_div() {
    assert_eq!(div(1.0, 2.0), 0.5);
}
