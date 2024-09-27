use crate::cli::config::{
    PFiles,
    PhinkFiles,
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
    fn from_fullpath(log_fullpath: PathBuf) -> anyhow::Result<Self::Output> {
        match log_fullpath.exists() {
            true => Ok(Self::create_instance(log_fullpath)),
            false => bail!("The fullpath isn't correct"),
        }
    }

    fn from_output(output: PathBuf) -> anyhow::Result<Self::Output> {
        let path = PhinkFiles::new(output).path(Self::get_pfile_type());

        match path.exists() {
            true => Self::from_fullpath(path),
            false => {
                bail!(format!("Couldn't spot {:?}", path))
            }
        }
    }

    fn create_instance(path: PathBuf) -> Self::Output;

    fn get_pfile_type() -> PFiles;
}
