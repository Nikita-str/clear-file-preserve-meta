use std::path::Path;
use std::fs::{File, Metadata};
use std::io::Write;

use super::{ResultIO, ClearFile, ClearDir};
use crate::filter::{FileFilter, DirFilter};


/// `ClearFile` that change content of a file to new const value
pub struct ConstChangeContF<'filter, S: AsRef<str>> {
    new_content: S,
    file_filter: &'filter FileFilter,
}

impl ConstChangeContF<'static, &'static str> {
    pub fn new_clear_all() -> Self {
        Self {
            new_content: "", 
            file_filter: &FileFilter::EMPTY,
        }
    }
}

impl<S: AsRef<str>> ConstChangeContF<'static, S> {
    pub fn new_no_filter(new_content: S) -> Self {
        Self { 
            new_content, 
            file_filter: &FileFilter::EMPTY,
        }
    }
}

impl<'filter, S: AsRef<str>> ConstChangeContF<'filter, S> {
    pub fn new(new_content: S, file_filter: &'filter FileFilter) -> Self {
        Self { 
            new_content, 
            file_filter,
        }
    }
}

impl<'filter, S: AsRef<str>> ClearFile for ConstChangeContF<'filter, S> {
    
    fn is_file_allow(&mut self, path: impl AsRef<Path>) -> bool {
        self.file_filter.is_allowed_unchecked(path).unwrap_or(false)
    }

    fn clear_action(&mut self, f: &mut File, _: &Metadata) -> ResultIO {
        let new_cont = self.new_content.as_ref();
        if !new_cont.is_empty() {
            write!(f, "{new_cont}")?;
        }
        Ok(())
    }

}


/// `ClearDir` that change content of a file in a dir to new const value
/// 
/// by default is not recursive  
pub struct ConstChangeContD<'filter, S: AsRef<str>> {
    file_chg: ConstChangeContF<'filter, S>,
    dir_filter: &'filter DirFilter,
    recursive: bool,
}

impl ConstChangeContD<'static, &'static str> {
    pub fn new_clear_all() -> Self {
        Self {
            file_chg: ConstChangeContF::new_clear_all(),
            dir_filter: &DirFilter::EMPTY,
            recursive: false,
        }
    }
}

impl<S: AsRef<str>> ConstChangeContD<'static, S> {
    pub fn new_no_filter(new_content: S) -> Self {
        Self {
            file_chg: ConstChangeContF::new_no_filter(new_content),
            dir_filter: &DirFilter::EMPTY,
            recursive: false,
        }
    }
}

impl<'filter, S: AsRef<str>> ConstChangeContD<'filter, S> {
    pub fn new_no_file_filter(new_content: S, dir_filter: &'filter DirFilter) -> Self {
        Self {
            file_chg: ConstChangeContF::new_no_filter(new_content),
            dir_filter,
            recursive: false,
        }
    }

    pub fn new_no_dir_filter(new_content: S, file_filter: &'filter FileFilter) -> Self {
        Self {
            file_chg: ConstChangeContF::new(new_content, file_filter),
            dir_filter: &DirFilter::EMPTY,
            recursive: false,
        }
    }

    pub fn new(
        new_content: S, 
        file_filter: &'filter FileFilter, 
        dir_filter: &'filter DirFilter,
    ) -> Self {
        Self {
            file_chg: ConstChangeContF::new(new_content, file_filter),
            dir_filter,
            recursive: false,
        }
    }

    pub fn set_recursive(&mut self, recursive: bool) {
        self.recursive = recursive
    }
}

impl<'filter, S: AsRef<str>> ClearFile for ConstChangeContD<'filter, S> {
    fn is_file_allow(&mut self, path: impl AsRef<Path>) -> bool {
        self.file_chg.is_file_allow(path)
    }

    fn clear_action(&mut self, f: &mut File, md: &Metadata) -> ResultIO {
        self.file_chg.clear_action(f, md)
    }
}

impl<'filter, S: AsRef<str>> ClearDir for ConstChangeContD<'filter, S> {
    fn is_dir_allow(&mut self, dir_path: impl AsRef<Path>) -> bool {
        self.dir_filter.is_allowed_unchecked(dir_path)
    }

    fn is_recursive(&mut self, _: impl AsRef<Path>) -> bool {
        self.recursive
    }
}