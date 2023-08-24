pub struct Cli {}

impl Cli {
    pub fn run() -> Result<(), crate::Error> {
        std::fs::create_dir(LOG_DIR.clone().unwrap_or("".into())).unwrap_or_else(|_|{});

        let file_appender = tracing_appender::rolling::hourly(LOG_DIR.clone().unwrap_or("".into()), *LOGFILE_NAME);
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
        tracing::info!("Hello!");
        Ok(())
    }
    
}

// region: IMPORTS

use crate::{LOGFILE_NAME, LOG_DIR};
use tracing_subscriber::layer::SubscriberExt;

// endregion: IMPORTS

// region: MODULES

mod cli_template {
    #[derive(clap::Parser, Debug)]
    pub struct CliTemplate {
        #[clap(flatten)]
        verbose: clap_verbosity_flag::Verbosity,
    }
}

//endregion: MODULES
