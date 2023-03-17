#[cfg(test)]
mod tests;
mod file_filter;
pub use file_filter::FileFilter;

pub use fns::{clear_file_filter_f, clear_file, change_file_cont_filter_f, change_file_content}; 
pub use fns::{clear_dir_files_filter_f, clear_dir_files, change_dir_files_cont_filter_f, change_dir_files_content};

mod fns {
    use std::io::Write;
    use std::path::Path;
    use std::fs::{File, Metadata};
    use filetime::FileTime;
    
    use crate::FileFilter;

    // use std::io::Result as ResultIO;
    type ResultIO<T> = std::io::Result<T>;


    fn change_content_action(f: &mut File, new_content: &str) -> ResultIO<()> {
        if !new_content.is_empty() {
            write!(f, "{new_content}")?;
        }
        Ok(())
    }
    macro_rules! change_content_act {
        ($new_content:ident) => {
            |f: &mut File, _: &Metadata| change_content_action(f, $new_content)
        };
    }


    pub fn clear_file_filter_f<F>(
        path: impl AsRef<Path>, 
        clear_action: F, 
        file_filter: &FileFilter,
    ) -> ResultIO<()>
    where F: FnOnce(&mut File, &Metadata) -> ResultIO<()>
    {
        let path = path.as_ref();
        
        // `_unchecked` because it checks later (see [link:1] and [link:2])
        if !file_filter.is_allowed_unchecked(path).unwrap_or(false) { return Ok(()) }

        let md = std::fs::metadata(path)?; // [link:1]
        let mtime = FileTime::from_system_time(md.modified()?);
        let atime = FileTime::from_system_time(md.accessed()?);

        {
            let mut f = File::create(path)?; // [link:2]
            clear_action(&mut f, &md)?;
        }

        filetime::set_file_times(path, atime, mtime)?;

        Ok(())
    }

    pub fn clear_file<F>(path: impl AsRef<Path>, clear_action: F) -> ResultIO<()>
    where F: FnOnce(&mut File, &Metadata) -> ResultIO<()>
    {
        clear_file_filter_f(path, clear_action, &FileFilter::EMPTY)
    }
    
    pub fn change_file_cont_filter_f(
        path: impl AsRef<Path>, 
        new_content: &str,
        file_filter: &FileFilter,
    ) -> ResultIO<()> {
        let clear_action = change_content_act!(new_content);
        clear_file_filter_f(path, clear_action, file_filter)
    }

    pub fn change_file_content(path: impl AsRef<Path>, new_content: &str) -> ResultIO<()> {
        change_file_cont_filter_f(path, new_content, &FileFilter::EMPTY)
    }

    //TODO: pub fn clear_file_save_byte_size


    pub fn clear_dir_files_filter_f<F>(
        path: impl AsRef<Path>, 
        clear_action: F, 
        recursive: bool,
        file_filter: &FileFilter,
    ) -> ResultIO<()>
    where for <'x> &'x F: FnMut(&mut File, &Metadata) -> ResultIO<()>
    {
        let mut rec_dirs = vec![];
        let mut first = true;

        // loop for recursive dir traversal
        loop {
            let dir_elems = if first {
                std::fs::read_dir(path.as_ref())
            } else {
                let path = rec_dirs.pop();
                if let Some(path) = path { std::fs::read_dir(path) }
                else { break }
            }?;

            for dir_elem in dir_elems {
                let dir_elem = dir_elem?;
                let path = dir_elem.path();
                
                if recursive && path.is_dir() { 
                    rec_dirs.push(path)
                } else if path.is_file() { 
                    clear_file_filter_f(path, &clear_action, file_filter)?
                }
            }

            first = false;
        }

        Ok(())
    }

    pub fn clear_dir_files<F>(path: impl AsRef<Path>, clear_action: F, recursive: bool) -> ResultIO<()>
    where for <'x> &'x F: FnMut(&mut File, &Metadata) -> ResultIO<()>
    {
        clear_dir_files_filter_f(path, clear_action, recursive, &FileFilter::EMPTY)
    }
    
    pub fn change_dir_files_cont_filter_f(
        path: impl AsRef<Path>,
        new_content: &str,
        recursive: bool,
        file_filter: &FileFilter,
    ) -> ResultIO<()> {
        let clear_action = change_content_act!(new_content);
        clear_dir_files_filter_f(path, clear_action, recursive, file_filter)
    }

    pub fn change_dir_files_content(path: impl AsRef<Path>, new_content: &str, recursive: bool) -> ResultIO<()> {
        change_dir_files_cont_filter_f(path, new_content, recursive, &FileFilter::EMPTY)
    }
}
