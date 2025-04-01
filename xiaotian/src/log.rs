use tracing::{Level, info};
use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn init_logger() {
    let info = tracing_appender::rolling::daily("logs", "info.log").with_max_level(Level::INFO);
    let error = tracing_appender::rolling::daily("logs", "error.log").with_max_level(Level::ERROR);
    let debug = tracing_appender::rolling::daily("logs", "debug.log").with_max_level(Level::DEBUG);

    tracing_subscriber::fmt()
        .with_writer(info)
        .with_writer(error)
        .with_writer(debug)
        .init();

    info!("xiaotian started");
}
