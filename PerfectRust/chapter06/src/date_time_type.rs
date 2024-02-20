use chrono::prelude::*; // make all chrono features available
#[allow(dead_code)]
pub fn instantiate() {
    let now: DateTime<Utc> = Utc::now();
    println!("UTC datetime: {}", &now);
    let now: DateTime<Local> = Local::now();
    println!("Local datetime: {}", &now);
}

#[allow(dead_code)]
pub fn format() {
    let now: DateTime<Utc> = Utc::now();
    let format_date = now.format("%Y年%m月%d日").to_string();
    println!("{:?}", &format_date);
    let now: DateTime<Local> = Local::now();
    let format_date = now.format("%Y年%m月%d日 %H時%M分:%S秒").to_string();
    println!("{:?}", &format_date);
}
