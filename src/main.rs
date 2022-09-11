mod numerals;

use numerals::Numerals;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let val = args[1].parse::<u32>();
    if let Err(e) = val {
        println!("{:?}", e);
        return;
    }

    let val = val.unwrap();

    let nums = Numerals::new(val);
    nums.print();

}
