use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
pub fn test_loop() {
    loop {
        println!("again!");
        sleep(Duration::from_secs(1));
    }
}

// -------------------------------------------------

#[allow(dead_code)]
pub fn test_loop_statement() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// -------------------------------------------------

#[allow(dead_code)]
pub fn test_loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

// -------------------------------------------------

#[allow(dead_code)]
pub fn test_loop_statement_label() {
    let mut count = 0;
    let ans = 'counting_up: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up count * 2;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("The ans is {}", ans);
}

// -------------------------------------------------

#[allow(dead_code)]
pub fn test_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
