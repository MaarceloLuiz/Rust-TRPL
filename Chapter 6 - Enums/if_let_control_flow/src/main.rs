fn main() {
    /*
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),           // contains too much boiler plate since we have to satisfy the match expression by adding "_ => ()"
    }
     */

     let config_max = Some(3u8);
     if let Some(max) = config_max {
         println!("The maximum is configured to be {max}");
     }
 
}
