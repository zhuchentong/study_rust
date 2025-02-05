use crate::modules_2::structs::HousePrice;
use csv::*;

pub fn read_csv(path: String) -> Vec<HousePrice> {
    let mut rdr = Reader::from_path(path).unwrap();
    vec![]
}
