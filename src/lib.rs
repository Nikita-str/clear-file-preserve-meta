#[cfg(test)]
mod tests;

pub use fns::{clear_file, change_file_content}; 
pub use fns::{clear_dir_files, change_dir_files_content};

mod fns {
    use std::io::Write;
    use std::path::Path;
    use std::fs::{File, Metadata};
    use filetime::FileTime;

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


    pub fn clear_file<F>(path: impl AsRef<Path>, clear_action: F) -> ResultIO<()>
    where F: FnOnce(&mut File, &Metadata) -> ResultIO<()>
    {
        let path = path.as_ref();

        let md = std::fs::metadata(path)?;
        let mtime = FileTime::from_system_time(md.modified()?);
        let atime = FileTime::from_system_time(md.accessed()?);

        {
            let mut f = File::create(path)?;
            clear_action(&mut f, &md)?;
        }

        filetime::set_file_times(path, atime, mtime)?;

        Ok(())
    }

    pub fn change_file_content(path: impl AsRef<Path>, new_content: &str) -> ResultIO<()> {
        let clear_action = change_content_act!(new_content);
        clear_file(path, clear_action)
    }

    //TODO: pub fn clear_file_save_byte_size


    pub fn clear_dir_files<F>(path: impl AsRef<Path>, clear_action: F, recursive: bool) -> ResultIO<()>
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
                    clear_file(path, &clear_action)?
                }
            }

            first = false;
        }

        Ok(())
    }

    pub fn change_dir_files_content(path: impl AsRef<Path>, new_content: &str, recursive: bool) -> ResultIO<()> {
        let clear_action = change_content_act!(new_content);
        clear_dir_files(path, clear_action, recursive)
    }
}
