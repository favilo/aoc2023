use anyhow::Result;
use clap::{ArgAction, Parser};
use fern::colors::{Color, ColoredLevelConfig};

fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                // use builder methods
                .info(Color::Green)
                .warn(Color::Magenta);
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, action=ArgAction::Append)]
    days: Vec<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let days: Vec<usize> = args.days;
    setup_logger()?;

    let time = aoc2022::run(days)?;
    log::info!("Total Time: {:?}", time);

    Ok(())
}
