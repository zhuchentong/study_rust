use super::enums::YesNo;

#[derive(Debug)]
pub struct HousePrice {
    pub price: i32,
    pub name: String,
    pub main_road: YesNo,
}
