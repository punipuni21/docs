#[allow(dead_code)]
pub fn branch_1() {
    let x = 10;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        _ => println!("value is invalid"),
    }

    match x {
        1 => {
            let y = 100;
            println!("y={}", y);
        }
        2 => {
            let y = 200;
            println!("y={}", y);
        }
        _ => {
            let y = 300;
            println!("y={}", y);
        }
    }
}

#[allow(dead_code)]
pub fn branch_2() {
    let x = "ABC";
    match x {
        "ABC" => println!("x is ABC"),
        "DEF" => println!("x is DEF"),
        _ => println!("x is invalid"),
    }
}
