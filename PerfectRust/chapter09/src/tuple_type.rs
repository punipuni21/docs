#[derive(Debug)]
pub enum Season2<'a> {
    Spring(u8, Vec<&'a str>),
    Summer(u8, Vec<&'a str>),
    Autumn(u8, Vec<&'a str>),
    Winter(u8, Vec<&'a str>),
}

impl<'a> Season2<'a> {
    pub fn format_valiant(&self) -> String {
        match self {
            Self::Spring(x, y) => format!("spring:{} months {:?}", x, y),
            Self::Summer(x, y) => format!("summer:{} months {:?}", x, y),
            Self::Autumn(x, y) => format!("autumn:{} months {:?}", x, y),
            Self::Winter(x, y) => format!("winter:{} months {:?}", x, y),
        }
    }
}

#[allow(dead_code)]
pub fn use_tuple() {
    let spring = Season2::Spring(3, vec!["March", "April", "May"]);
    let summer = Season2::Summer(3, vec!["June", "July", "August"]);
    let autumn = Season2::Autumn(3, vec!["September", "October", "November"]);
    let winter = Season2::Winter(3, vec!["December", "January", "February"]);
    println!("{}", spring.format_valiant());
    println!("{}", summer.format_valiant());
    println!("{}", autumn.format_valiant());
    println!("{}", winter.format_valiant());
}
