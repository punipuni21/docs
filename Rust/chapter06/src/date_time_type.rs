use chrono::prelude::*; // make all chrono features available
#[allow(dead_code)]
pub fn instantiate() {
    let now: DateTime<Utc> = Utc::now();
    println!("UTC datetime: {}", &now);
    let now: DateTime<Local> = Local::now();
    println!("Local datetime: {}", &now);
}
