extern crate cargo_check;

use cargo_check::hello;

fn main() {
    println!("{}", hello());
}

#[cfg(feature = "feature")]
#[test]
fn test_app() {
    println!("--- App with feature, library: {} ---", hello());
    assert_eq!("feature", hello())
}

#[cfg(not(feature = "feature"))]
#[test]
fn test_app() {
    println!("--- App with no feature, library: {} ---", hello());
    assert_eq!("no feature", hello())
}
