extern crate chrono;
pub extern crate nalgebra as na;

pub use log::{debug, error, info, trace, warn};

pub type Result<T> = std::result::Result<T, String>;

mod scene;
pub use crate::scene::{Camera, Object, Scene, Transform};

fn setup_logger() -> std::result::Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::White)
        .debug(fern::colors::Color::Blue)
        .trace(fern::colors::Color::Magenta);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("pathmarch.log")?)
        .apply()?;
    Ok(())
}

pub fn render() -> Result<()> {
    match setup_logger() {
        Err(e) => Err(format!("{}", e)),
        _ => Ok(()),
    }?;
    trace!("Trace message");
    debug!("DBG message");
    info!("Beginning render");
    warn!("Warnning msg");
    error!("Error message");
    Ok(())
}
