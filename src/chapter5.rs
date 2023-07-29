//! Chapter 5
//!
//! 構造体を使用して関係のあるデータを構造化する

struct User {
    name: String,
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn greet(&self) -> String {
        format!("Hi! My name is {}", self.name)
    }
}

struct Color(i32, i32, i32);

impl Color {
    fn is_black(&self) -> bool {
        if self.0 == self.1 && self.1 == self.2 && self.2 == 0 {
            return true
        }
        false
    }
    fn is_white(&self) -> bool {
        if self.0 == self.1 && self.1 == self.2 && self.2 == 255 {
            return true
        }
        false
    }
}

pub fn __main__() {
    let mut user1 = build_user(
        String::from("John"),
        String::from("john"),
        String::from("example@example.com"));

    let _black = Color(0, 0, 0);

    user1.username = String::from("jane");

    println!("{}", user1.greet());
}

/// Build a user
///
fn build_user(name: String, username: String, email: String) -> User {
    User {
        name,
        username,
        email,
        active: true
    }
}

#[test]
fn test_is_white() {
    let c = Color(0, 0, 0);
    assert_eq!(c.is_white(), false);
}

#[test]
fn test_is_black() {
    let c = Color(0, 0, 0);
    assert_eq!(c.is_black(), true);
}
