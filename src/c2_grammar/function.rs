#[allow(dead_code)]
fn five() -> i32{
    5
}

#[allow(dead_code)]
fn six() -> i32 {
    return 6
}

#[allow(dead_code)]
pub fn test_function_return() {
    println!("5 + 6 = {}", five() + six());
}

// -------------------------------------------------

#[allow(dead_code)]
fn add_one(x: i32) -> i32{
    x + 1
}

#[allow(dead_code)]
pub fn test_function_param() {
    println!("1 + 2 = {}", add_one(2));
}
