use std::fmt::Display;
use std::fmt::Formatter;

#[repr(u32)]
#[allow(dead_code)]
#[derive(Debug)]
pub enum Season {
    Spring = 100,
    Summer = 200,
    Autumn,
    Winter,
}

impl Display for Season {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Spring => write!(f, "Spring : {}", Self::Spring as u32),
            Self::Summer => write!(f, "Summer : {}", Self::Summer as u32),
            Self::Autumn => write!(f, "Autumn : {}", Self::Autumn as u32),
            Self::Winter => write!(f, "Winter : {}", Self::Winter as u32),
        }
    }
}

#[allow(dead_code)]
pub fn use_season() {
    let summer = Season::Summer;
    let winter = Season::Winter;
    println!("Summer is {:?}", summer); //cannot print with {}
    println!("Winter is {:?}", winter);
    //convert into integer
    let summer_num = Season::Summer as i32;
    let winter_num = Season::Winter as i32;
    println!("Summer is {}", summer_num);
    println!("Winter is {}", winter_num);
}

#[allow(dead_code)]
pub fn use_fmt() {
    println!("{}", Season::Summer);
    println!("{}", Season::Autumn);
}

#[allow(dead_code)]
pub fn use_repr() {
    println!("{}", Season::Spring);
    println!("{}", Season::Summer);
    println!("{}", Season::Autumn);
    println!("{}", Season::Winter);
}
