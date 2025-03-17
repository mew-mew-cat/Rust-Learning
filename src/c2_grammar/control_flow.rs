#[allow(dead_code)]
pub fn test_if_expr_1() {
    let number = 6;
    if number % 4 == 0 {
        println!("divisible by 7");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 4, 3, or 2");
    }
}

// -------------------------------------------------

#[allow(dead_code)]
pub fn test_if_expr_2() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}