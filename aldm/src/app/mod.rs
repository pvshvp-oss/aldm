lazy_static! {
    pub static ref APP_NAME: &'static str = "aldm";
}
pub trait Permissions<'a, P> {
    type P1;

    fn is_readable(&self) -> bool;

    fn is_writable(&self) -> bool;

    fn is_creatable(&self) -> bool;

    fn largest_valid_subset(&'a self) -> Option<Self::P1>;
}

impl<'a, P> Permissions<'a, P> for P
where
    P: AsRef<Path>,
{
    type P1 = &'a Path;

    fn is_readable(&self) -> bool {
        permissions::is_readable(self).unwrap_or(false)
    }

    fn is_writable(&self) -> bool {
        permissions::is_writable(self).unwrap_or(false)
    }

    fn is_creatable(&self) -> bool {
        match self.largest_valid_subset() {
            Some(p) => p.is_writable(),
            None => false,
        }
    }

    fn largest_valid_subset(&'a self) -> Option<Self::P1> {
        let mut path = self.as_ref();
        while !path.exists() {
            match path.parent() {
                Some(p) => path = p,
                None => {
                    return None;
                }
            };
        }

        Some(path)
    }
}

impl<'a, P> Permissions<'a, P> for Option<P>
where
    P: AsRef<Path>,
{
    type P1 = &'a Path;

    fn is_readable(&self) -> bool {
        match self {
            Some(p) => p.is_readable(),
            None => false,
        }
    }

    fn is_writable(&self) -> bool {
        match self {
            Some(p) => p.is_writable(),
            None => false,
        }
    }

    fn is_creatable(&self) -> bool {
        match self {
            Some(p) => p.is_creatable(),
            None => false,
        }
    }

    fn largest_valid_subset(&'a self) -> Option<Self::P1> {
        match self {
            Some(p) => p.largest_valid_subset(),
            None => None,
        }
    }
}

pub trait ValidPaths<'a, P, Q>
where
    P: AsRef<Path> + 'a,
{
    fn first_readable_path(&mut self) -> Option<P>;

    fn first_writable_path(&mut self) -> Option<P>;

    fn first_creatable_path(&mut self) -> Option<P>;

    fn all_readable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a>;

    fn all_writable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a>;

    fn all_creatable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a>;

    fn first_valid_path(&'a mut self, f: fn(&Q) -> bool) -> Option<P>;

    fn all_valid_paths(&'a mut self, f: fn(&Q) -> bool) -> Box<dyn Iterator<Item = P> + 'a>;
}

impl<'a, P, I> ValidPaths<'a, P, P> for I
where
    I: Iterator<Item = P> + 'a,
    P: AsRef<Path> + 'a,
{
    fn first_readable_path(&mut self) -> Option<P> {
        self.first_valid_path(P::is_readable)
    }

    fn first_writable_path(&mut self) -> Option<P> {
        self.first_valid_path(P::is_writable)
    }

    fn first_creatable_path(&mut self) -> Option<P> {
        self.first_valid_path(P::is_creatable)
    }

    fn all_readable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a> {
        self.all_valid_paths(P::is_readable)
    }

    fn all_writable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a> {
        self.all_valid_paths(P::is_writable)
    }

    fn all_creatable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a> {
        self.all_valid_paths(P::is_creatable)
    }

    fn first_valid_path(&mut self, f: fn(&P) -> bool) -> Option<P> {
        self.find(|p| f(p))
    }

    fn all_valid_paths(&'a mut self, f: fn(&P) -> bool) -> Box<dyn Iterator<Item = P> + 'a> {
        Box::new(self.filter(move |p| f(p)))
    }
}

impl<'a, P, I> ValidPaths<'a, P, Option<P>> for I
where
    I: Iterator<Item = Option<P>> + 'a,
    P: AsRef<Path> + 'a,
{
    fn first_readable_path(&mut self) -> Option<P> {
        self.first_valid_path(Option::<P>::is_readable)
    }

    fn first_writable_path(&mut self) -> Option<P> {
        self.first_valid_path(Option::<P>::is_writable)
    }

    fn first_creatable_path(&mut self) -> Option<P> {
        self.first_valid_path(Option::<P>::is_creatable)
    }

    fn all_readable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a> {
        self.all_valid_paths(Option::<P>::is_readable)
    }

    fn all_writable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a> {
        self.all_valid_paths(Option::<P>::is_writable)
    }

    fn all_creatable_paths(&'a mut self) -> Box<dyn Iterator<Item = P> + 'a> {
        self.all_valid_paths(Option::<P>::is_creatable)
    }

    fn first_valid_path(&mut self, f: fn(&Option<P>) -> bool) -> Option<P> {
        self.find(|p| f(p))
            .flatten()
    }

    fn all_valid_paths(
        &'a mut self,
        f: fn(&Option<P>) -> bool,
    ) -> Box<dyn Iterator<Item = P> + 'a> {
        Box::new(
            self.filter(move |p| f(p))
                .flat_map(convert::identity),
        )
    }
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("in logging: {source}"), visibility(pub))]
    Logging {
        #[snafu(backtrace)]
        source: logging::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in configuration: {source}"), visibility(pub))]
    Config {
        #[snafu(backtrace)]
        source: config::Error,
    },

    #[non_exhaustive]
    #[snafu(display("in internationalization: {source}"), visibility(pub))]
    Internationalization {
        #[snafu(backtrace)]
        source: i18n::Error,
    },

    #[non_exhaustive]
    #[snafu(
        display("could not retreive the XDG app directories for the user: {source}"),
        visibility(pub)
    )]
    FetchXdgAppDirectories { source: xdg::BaseDirectoriesError },
}

// region: IMPORTS

use lazy_static::lazy_static;
use snafu::Snafu;
use std::{convert, path::Path};

// endregion: IMPORTS

// region: MODULES

pub mod config;
pub mod i18n;
pub mod logging;

// endregion: MODULES

// region: RE-EXPORTS

pub use config::*;
pub use i18n::*;
pub use logging::*;

// endregion: RE-EXPORTS
