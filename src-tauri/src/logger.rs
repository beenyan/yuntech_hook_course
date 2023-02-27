use std::fs;

pub fn setup_logger() -> Result<(), fern::InitError> {
    fs::create_dir_all("assets/log/").unwrap();

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
        .level(log::LevelFilter::Info)
        .filter(|metadata| metadata.target() != "tao::platform_impl::platform::event_loop::runner")
        .chain(std::io::stdout())
        .chain(fern::log_file("assets/log/logger.log")?)
        .apply()?;

    Ok(())
}
