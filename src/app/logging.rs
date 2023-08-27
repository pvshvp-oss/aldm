pub fn init_log(log_level_filter: Option<LevelFilter>) -> Result<Handle, Error> {
    // Setup the log directory and log file(s)
    let log_dir_path = xdg::BaseDirectories::with_prefix(*APP_NAME)
        .context(BaseDirectoriesSnafu {})?
        .create_state_directory("")
        .context(LogDirectorySnafu {})?; // Setup the log directory
    let log_file_appender =
        tracing_appender::rolling::hourly(log_dir_path, format!("{}.log", *APP_NAME)); // Setup the log file

    // Obtain writers to various logging destinations and worker guards (for keeping the streams alive)
    let (non_blocking_file_writer, _file_writer_guard) =
        tracing_appender::non_blocking(log_file_appender);
    let (non_blocking_stdout_writer, _stdout_writer_guard) =
        tracing_appender::non_blocking(anstream::stdout());
    let (non_blocking_stderr_writer, _stderr_writer_guard) =
        tracing_appender::non_blocking(anstream::stderr());

    // Declare filtering rules for various logging destinations
    let filter_stdout = move |metadata: &Metadata<'_>| {
        metadata.level() <= &log_level_filter.unwrap_or(LevelFilter::INFO)
            && metadata.level() > &Level::WARN
            && !metadata
                .target()
                .eq_ignore_ascii_case("JSON")
    }; // Omit messages of higher verbosity than 'INFO', messages of equal or lower verbosity than 'WARN', and JSON target messages.
    let filter_stdout_json = move |metadata: &Metadata<'_>| {
        metadata.level() == &Level::INFO
            && metadata
                .target()
                .eq_ignore_ascii_case("JSON")
    }; // In JSON printing mode, print only 'INFO' messages, and permit JSON target messages.
    let stdout_filter: FilterFn<Box<dyn Fn(&Metadata<'_>) -> bool + Send + Sync>> =
        filter_fn(Box::new(filter_stdout)); // Box the closure to allow for type match when switching between two similar closures.
    let stdout_json_filter: FilterFn<Box<dyn Fn(&Metadata<'_>) -> bool + Send + Sync>> =
        filter_fn(Box::new(filter_stdout_json)); // Box the closure to allow for type match when switching between two similar closures.
    let (stdout_filter, stdout_filter_reload_handle) = reload::Layer::new(stdout_filter); // Wrap the filter in reload::Layer and obtain handle to allow switching between filters.

    // Closure to switch to JSON logging
    let switch_to_json = move || {
        stdout_filter_reload_handle.modify(
            |filter: &mut FilterFn<Box<dyn Fn(&Metadata<'_>) -> bool + Send + Sync>>| {
                *filter = stdout_json_filter
            },
        )
    };

    // Declare logging formats for various logging destinations
    let log_file_layer = fmt::Layer::new()
        .pretty()
        .with_ansi(true)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_target(true)
        .with_writer(non_blocking_file_writer)
        .with_filter(LevelFilter::TRACE);
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

    // Compose various filtered logging destination layers and set them to receive tracing messages
    let subscriber = tracing_subscriber::registry()
        .with(log_file_layer)
        .with(stdout_layer)
        .with(stderr_layer);
    tracing::subscriber::set_global_default(subscriber).context(GlobalDefaultSubscriberSnafu {})?;

    Ok(Handle {
        _switch_to_json_inner: Some(Box::new(switch_to_json)),
        worker_guards: vec![
            _file_writer_guard,
            _stdout_writer_guard,
            _stderr_writer_guard,
        ],
    })
}

pub struct Handle {
    _switch_to_json_inner: Option<Box<dyn FnOnce() -> Result<(), reload::Error>>>,
    pub worker_guards: Vec<WorkerGuard>,
}

impl Handle {
    pub fn switch_to_json(&mut self) -> Result<(), Error> {
        Ok((self
            ._switch_to_json_inner
            .take()
            .unwrap())()
        .context(JsonSwitchSnafu {})?)
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

    #[non_exhaustive]
    #[snafu(
        display("could not switch to JSON output format: {source}"),
        visibility(pub)
    )]
    JsonSwitch {
        source: tracing_subscriber::reload::Error,
    },
}

// region: IMPORTS

use crate::app::APP_NAME;
use snafu::{ResultExt, Snafu};
use tracing::{Level, Metadata};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    filter::{filter_fn, FilterFn, LevelFilter},
    fmt,
    layer::SubscriberExt,
    reload, Layer,
};

// endregion: IMPORTS
