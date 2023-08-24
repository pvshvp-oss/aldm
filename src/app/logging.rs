pub trait InitLog {
    fn init_log() {
        std::fs::create_dir("/tmp/aldm").unwrap_or_else(|_|{});

        let file_appender = tracing_appender::rolling::hourly("/tmp/aldm", "aldm.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let collector = tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing::Level::TRACE.into()),
            )
            .with(tracing_subscriber::fmt::Layer::new().with_writer(std::io::stdout))
            .with(tracing_subscriber::fmt::Layer::new().with_writer(non_blocking));
        tracing::subscriber::set_global_default(collector)
            .expect("Unable to set a global collector");
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use tracing_subscriber::layer::SubscriberExt;
use snafu::Snafu;

// endregion: IMPORTS
