use std::path::Path;
use std::fs::{File, Metadata};
use std::io::Write;
use crate::FileFilter;

pub trait ClearAct {
    type ClearError;

    fn is_file_allow(&mut self, path: impl AsRef<Path>) -> bool;
    fn clear_action(&mut self, f: &mut File, md: &Metadata) -> Result<(), Self::ClearError>;
}

/// `ClearAct` that change content to new const value
pub struct ConstChangeContent<'filter, S: AsRef<str>> {
    new_content: S,
    file_filter: &'filter FileFilter,
}

impl ConstChangeContent<'static, &'static str> {
    pub fn new_clear_all() -> Self {
        Self {
            new_content: "", 
            file_filter: &FileFilter::EMPTY,
        }
    }
}

impl<S: AsRef<str>> ConstChangeContent<'static, S> {
    pub fn new_no_filter(new_content: S) -> Self {
        Self { 
            new_content, 
            file_filter: &FileFilter::EMPTY,
        }
    }
}

impl<'filter, S: AsRef<str>> ConstChangeContent<'filter, S> {
    pub fn new(new_content: S, file_filter: &'filter FileFilter) -> Self {
        Self { 
            new_content, 
            file_filter,
        }
    }
}

impl<'filter, S: AsRef<str>> ClearAct for ConstChangeContent<'filter, S> {
    type ClearError = std::io::Error;
    
    fn is_file_allow(&mut self, path: impl AsRef<Path>) -> bool {
        self.file_filter.is_allowed_unchecked(path).unwrap_or(false)
    }

    fn clear_action(&mut self, f: &mut File, _: &Metadata) -> Result<(), Self::ClearError> {
        let new_cont = self.new_content.as_ref();
        if !new_cont.is_empty() {
            write!(f, "{new_cont}")?;
        }
        Ok(())
    }

}
