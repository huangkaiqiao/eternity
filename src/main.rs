use std::cmp;
use std::collections::HashMap;
// use log::{debug, error, info, trace, warn};
use log::info;

use eternity::Solution;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    // println!("Hello, world!");
    // let s = Solution{};
    let result = setup_logger();
    info!("{:?}", result.unwrap());
    // println!("{}", );
    // let r = Solution::minimum_delete_sum(String::from("sea"), String::from("eat"));
    // info!("{}", r);
    // let r = Solution::minimum_delete_sum(String::from("delete"), String::from("leet"));
    // info!("{}", r);
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee:i32 = 8;
    let r = Solution::max_profit(prices, fee);
    info!("{}", r);
}
