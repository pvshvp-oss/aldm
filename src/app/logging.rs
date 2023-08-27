pub trait InitLog {
    fn stdout_filter_fn(
        log_level_filter: Option<LevelFilter>,
    ) -> Box<dyn Fn(&Metadata<'_>) -> bool + Send + Sync> {
        Box::new(move |metadata: &Metadata<'_>| {
            metadata.level() <= &log_level_filter.unwrap_or(LevelFilter::INFO)
                && metadata.level() > &Level::WARN
                && !metadata
                    .target()
                    .eq_ignore_ascii_case("JSON")
        })
    }

    fn stdout_json_filter_fn() -> Box<dyn Fn(&Metadata<'_>) -> bool + Send + Sync> {
        Box::new(move |metadata: &Metadata<'_>| {
            metadata.level() == &Level::INFO
                && metadata
                    .target()
                    .eq_ignore_ascii_case("JSON")
        })
    }

    fn init_log(log_level_filter: Option<LevelFilter>) -> Result<Handle, Error> {
        let log_dir_path = xdg::BaseDirectories::with_prefix(*APP_NAME)
            .context(BaseDirectoriesSnafu {})?
            .create_state_directory("")
            .context(LogDirectorySnafu {})?;

        let file_appender =
            tracing_appender::rolling::hourly(log_dir_path, format!("{}.log", *APP_NAME));
        let (non_blocking_file_writer, _file_writer_guard) =
            tracing_appender::non_blocking(file_appender);
        let (non_blocking_stdout_writer, _stdout_writer_guard) =
            tracing_appender::non_blocking(anstream::stdout());
        let (non_blocking_stderr_writer, _stderr_writer_guard) =
            tracing_appender::non_blocking(anstream::stderr());

        let log_file_layer = fmt::Layer::new()
            .pretty()
            .with_ansi(true)
            .with_file(true)
            .with_level(true)
            .with_line_number(true)
            .with_target(true)
            .with_writer(non_blocking_file_writer)
            .with_filter(LevelFilter::TRACE);
        let stdout_filter = filter_fn(Self::stdout_filter_fn(log_level_filter));
        let (stdout_filter, stdout_filter_reload_handle) = reload::Layer::new(stdout_filter);
        let stdout_layer = fmt::Layer::new()
            .with_ansi(true)
            .with_file(false)
            .with_level(false)
            .with_line_number(false)
            .with_target(false)
            .without_time()
            .with_writer(non_blocking_stdout_writer)
            .with_filter(stdout_filter);
        let stderr_layer = fmt::Layer::new()
            .with_ansi(true)
            .with_file(false)
            .with_level(true)
            .with_line_number(false)
            .with_target(false)
            .without_time()
            .with_writer(non_blocking_stderr_writer)
            .with_filter(LevelFilter::WARN);
        let subscriber = tracing_subscriber::registry()
            .with(log_file_layer)
            .with(stdout_layer)
            .with(stderr_layer);
        tracing::subscriber::set_global_default(subscriber)
            .context(GlobalDefaultSubscriberSnafu {})?;

        Ok(Handle {
            switch_to_json_inner: Some(Box::new(move || {
                {
                    stdout_filter_reload_handle.modify(
                        |filter: &mut FilterFn<
                            Box<dyn Fn(&Metadata<'_>) -> bool + Send + Sync>,
                        >| {
                            *filter = filter_fn(Self::stdout_json_filter_fn())
                        },
                    )
                }
                .unwrap()
            })),
            worker_guards: vec![
                _file_writer_guard,
                _stdout_writer_guard,
                _stderr_writer_guard,
            ],
        })
    }
}

pub struct Handle {
    pub switch_to_json_inner: Option<Box<dyn FnOnce() -> ()>>,
    pub worker_guards: Vec<WorkerGuard>,
}

impl Handle {
    pub fn switch_to_json(&mut self) {
        (self
            .switch_to_json_inner
            .take()
            .unwrap())()
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(
        display("could not retreive XDG base directories': {source}"),
        visibility(pub)
    )]
    BaseDirectories { source: xdg::BaseDirectoriesError },

    #[non_exhaustive]
    #[snafu(
        display("could not create the log directory: {source}"),
        visibility(pub)
    )]
    LogDirectory { source: std::io::Error },

    #[non_exhaustive]
    #[snafu(
        display("could not set the global default tracing subscriber: {source}"),
        visibility(pub)
    )]
    GlobalDefaultSubscriber {
        source: tracing::subscriber::SetGlobalDefaultError,
    },
}

// region: IMPORTS

use crate::app::APP_NAME;
use snafu::{ResultExt, Snafu};
use tracing::{Level, Metadata};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    filter::{filter_fn, FilterFn, LevelFilter},
    fmt::{self},
    layer::SubscriberExt,
    reload, Layer,
};

// endregion: IMPORTS
