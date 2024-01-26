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

#[allow(dead_code)]
pub fn branch_3() {
    let calc = |x: i32| x * 10;
    let y = 3;
    let result = match y {
        1 => calc(10),
        2 => calc(20),
        3 => calc(30),
        _ => calc(0),
    };
    println!("result={}", result);
}

pub fn branch_4() {
    let calc = |x: i32| x * 10;
    let value = 30;
    let result = match value {
        1..=3 => calc(10),
        4..=6 => calc(20),
        7..=9 => calc(30),
        10 | 20 | 30 => calc(40),
        _ => calc(0),
    };
    println!("result={}", result);
}
