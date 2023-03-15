#[cfg(test)]
mod tests;

pub use fns::{clear_file, change_file_content};

mod fns {
    use std::io::Write;
    use std::path::Path;
    use std::fs::{File, Metadata};
    use filetime::FileTime;

    // use std::io::Result as ResultIO;
    type ResultIO<T> = std::io::Result<T>;


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
        let clear_action = |f: &mut File, _: &Metadata| {
            if !new_content.is_empty() {
                write!(f, "{new_content}")?;
            }
            Ok(())
        };

        clear_file(path, clear_action)
    }

}
