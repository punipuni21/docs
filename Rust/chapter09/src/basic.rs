#[allow(dead_code)]
#[derive(Debug)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[allow(dead_code)]
pub fn use_season() {
    let summer = Season::Summer;
    let winter = Season::Winter;
    println!("Summer is {:?}", summer);
    println!("Winter is {:?}", winter);
    //convert into integer
    let summer_num = Season::Summer as i32;
    let winter_num = Season::Winter as i32;
    println!("Summer is {}", summer_num);
    println!("Winter is {}", winter_num);
}
