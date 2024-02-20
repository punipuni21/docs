#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Season<T> {
    Spring(u8, T),
    Summer(u8, T),
    Autumn(u8, T),
    Winter(u8, T),
}

impl<T> Season<T> {
    pub fn get_months(&self) -> &T
    where
        T: std::iter::IntoIterator,
    {
        match self {
            Season::Spring(_, months) => months,
            Season::Summer(_, months) => months,
            Season::Autumn(_, months) => months,
            Season::Winter(_, months) => months,
        }
    }
}

#[allow(dead_code)]
pub fn use_generics() {
    use std::collections::LinkedList;

    let spring = Season::Spring(3, vec!["March", "April", "May"]);
    println!("spring: {:?}", spring.get_months());

    let summer = Season::Summer(3, vec!["June", "July", "August"]);
    println!("summer: {:?}", summer.get_months());

    let autumn = Season::Autumn(3, LinkedList::from(["September", "October", "November"]));
    println!("autumn: {:?}", autumn.get_months());
}
