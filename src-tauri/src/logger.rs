use crate::utils::ASSETS_DIR;
use std::fs;

pub fn setup_logger() -> Result<(), fern::InitError> {
    fs::create_dir_all(format!("{ASSETS_DIR}/log")).unwrap();

    #[cfg(debug_assertions)]
    let level = log::LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let level = log::LevelFilter::Info;

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
        .level(level)
        .filter(|metadata| metadata.target() != "tao::platform_impl::platform::event_loop::runner")
        .chain(std::io::stdout())
        .chain(fern::log_file(format!("{ASSETS_DIR}/log/logger.log"))?)
        .apply()?;

    Ok(())
}
