use std::path::Path;
use std::fs::{File, Metadata};
use filetime::FileTime;

use super::ResultIO;

pub trait ClearFile {
    /// should the file be cleared?
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

pub trait ClearDir: ClearFile {
    /// should the dir be cleared?
    fn is_dir_allow(&mut self, dir_path: impl AsRef<Path>) -> bool;
    /// should the dir be cleared recursively?
    fn is_recursive(&mut self, dir_path: impl AsRef<Path>) -> bool;

    fn clear_dir_files(&mut self, dir_path: impl AsRef<Path>) -> ResultIO
    {
        let dir_path = dir_path.as_ref();
        if !self.is_dir_allow(dir_path) { return Ok(()) }

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
