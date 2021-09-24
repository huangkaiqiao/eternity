// pub struct Solution;
mod problem118;
mod problem338;
mod problem509;
mod problem713;
mod problem714;
mod problem1025;

// pub use problem713::Solution as Solution;
// pub use problem714::Solution as Solution;
// pub use problem1025::Solution as Solution;
// pub use problem338::count_bits;

pub struct Solution{}

pub fn setup_logger() -> Result<(), fern::InitError> {
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