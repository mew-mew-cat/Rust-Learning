#[allow(dead_code)]
pub fn test_statement() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);
}