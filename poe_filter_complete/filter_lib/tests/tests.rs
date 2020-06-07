#[cfg(test)]
mod tests {
    // use filter_lib::logos_parsing;
    use filter_lib::data_parsing;
    use filter_lib::mode_parsing;
    #[test]
    fn test_new_filter_block() {
        let filter_file = include_str!("../src/test_filters/small.filter");
        let x = mode_parsing::parse(filter_file);

        // for b in x.iter(){
        //     for k in b.keywords.iter(){
        //  println!("{:#?}", k.value);

        //     }
        // }
        // println!("{:#?}", x);
        // for block in x.vec.iter() {
        //     println!("tokens :{:#?}", block.keywords.len());
        //     for line in block.keywords.iter() {
        //         println!("values :{:#?}", line.value.len());
        //     }
        // }
    }

    // #[test]
    // fn iterating_modes() {
    //     let s = include_str!("../src/test_filters/small.filter");
    //     let moded = mode_parsing::ModeBridge {
    //         mode: mode_parsing::Modes::new(s),
    //     };

    //     let results: Vec<mode_parsing::Tokens> = moded.collect();
    //     println!("{:?}", results)
    // }

    #[test]
    fn test_data_parsing() {
        let x = data_parsing::PoeData::new();
        println!("{:#?}", x)
    }
}
