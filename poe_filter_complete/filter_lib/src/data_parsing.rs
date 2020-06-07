// use csv::*;
use serde::Deserialize;

#[derive(Deserialize, Default, Clone, Debug)]
pub struct PoeData {
    pub classes: Vec<Record>,
    pub bases: Vec<Record>,
    pub mods: Vec<Record>,
}
#[derive(Deserialize, Default, Clone, Debug)]
pub struct Record {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub category: Option<String>,
    pub domain: Option<i32>,
    pub droplevel: Option<i32>,
}

// #[derive(Deserialize, Debug)]
// pub struct ItemClass {
// 	pub id: Option<i32>,
// 	pub name: Option<String>,
// 	pub category: Option<String>,
// }
// #[derive(Deserialize, Debug)]
// pub struct BaseType {
// 	pub name: Option<String>,
// 	pub class: Option<i32>,
// 	pub domain: Option<i32>,
// 	pub droplevel: Option<i32>,
// }
// #[derive(Deserialize, Debug)]
// pub struct Mod {
// 	pub domain: Option<i32>,
// 	pub name: Option<String>,
// }

impl PoeData {
    pub fn new() -> Self {
        let mut poe_data = PoeData {
            classes: vec![],
            bases: vec![],
            mods: vec![],
        };

        let classes = include_str!("test_filters/itemclasses.csv");
        let bases = include_str!("test_filters/baseitems.csv");
        let mods = include_str!("test_filters/mods.csv");

        let mut class = csv::Reader::from_reader(classes.as_bytes());
        let mut base = csv::Reader::from_reader(bases.as_bytes());
        let mut m = csv::Reader::from_reader(mods.as_bytes());

        for result in class.deserialize() {
            let record: Record = result.unwrap();
            poe_data.classes.push(record);
        }
        for result in base.deserialize() {
            let record: Record = result.unwrap();
            poe_data.bases.push(record);
        }
        for result in m.deserialize() {
            let record: Record = result.unwrap();
            poe_data.mods.push(record);
        }
        poe_data
    }
}
