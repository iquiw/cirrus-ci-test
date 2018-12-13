fn foo(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_main() {
    assert!(foo(2, 3) == 5);
}
