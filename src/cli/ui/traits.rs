use crate::{
    cli::config::{
        PFiles,
        PhinkFiles,
    },
    ResultOf,
};
use anyhow::bail;
use ratatui::{
    layout::Rect,
    Frame,
};
use std::path::PathBuf;

pub trait Paint {
    fn render(&self, f: &mut Frame, area: Rect);
}

pub trait FromPath {
    type Output;
    fn from_fullpath(fullpath: PathBuf) -> ResultOf<Self::Output> {
        match fullpath.exists() {
            true => Ok(Self::create_instance(fullpath)),
            false => bail!("The {fullpath:?} fullpath isn't correct"),
        }
    }

    fn from_output(output: PathBuf) -> ResultOf<Self::Output> {
        let path = PhinkFiles::new(output).path(Self::get_filetype());

        match path.exists() {
            true => Self::from_fullpath(path),
            false => bail!("Couldn't spot {path:?}"),
        }
    }

    fn create_instance(path: PathBuf) -> Self::Output;

    fn get_filetype() -> PFiles;
}
