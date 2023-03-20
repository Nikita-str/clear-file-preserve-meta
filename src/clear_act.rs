use std::path::Path;
use std::fs::{File, Metadata};
use std::io::Write;
use filetime::FileTime;

use crate::filter::FileFilter;

type ResultIO = std::io::Result<()>;


pub trait ClearAct {
    fn is_file_allow(&mut self, path: impl AsRef<Path>) -> bool;
    fn clear_action(&mut self, f: &mut File, md: &Metadata) -> ResultIO;

    fn clear_file(&mut self, path: impl AsRef<Path>) -> ResultIO {
        let path = path.as_ref();
                
        let md = std::fs::metadata(path)?;
        if !md.is_file() { return Ok(()) }
        let mtime = FileTime::from_system_time(md.modified()?);
        let atime = FileTime::from_system_time(md.accessed()?);
    
        if !self.is_file_allow(path) { return Ok(()) }

        {
            let mut f = File::create(path)?;
            self.clear_action(&mut f, &md)?;
        }
    
        filetime::set_file_times(path, atime, mtime)?;
    
        Ok(())
    }
}

pub trait ClearDir: ClearAct {
    fn is_dir_allow(&mut self, dir_path: impl AsRef<Path>) -> bool;
    fn is_recursive(&mut self, dir_path: impl AsRef<Path>) -> bool;

    fn clear_dir_files(&mut self, dir_path: impl AsRef<Path>) -> ResultIO
    {
        let dir_path = dir_path.as_ref();
        if self.is_dir_allow(dir_path) { return Ok(()) }

        let mut rec_dirs = vec![];
        let mut first = true;
    
        // loop for recursive dir traversal
        'rec: loop {
            let dir_elems = if first {
                std::fs::read_dir(dir_path)
            } else {
                let dir_path = rec_dirs.pop();
                if let Some(dir_path) = dir_path { 
                    if !self.is_dir_allow(&dir_path) { continue 'rec } 
                    std::fs::read_dir(dir_path) 
                } else {
                    break 'rec
                }
            }?;
    
            for dir_elem in dir_elems {
                let dir_elem = dir_elem?;
                let path = dir_elem.path();
                
                if path.is_dir() && self.is_recursive(&path) { 
                    rec_dirs.push(path)
                } else if path.is_file() { 
                    self.clear_file(path)?;
                }
            }
    
            first = false;
        }
    
        Ok(())
    }
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
