#[cfg(test)]
mod tests;

pub use fns::clear_file;

mod fns {
    use std::io::Write;
    use std::path::Path;
    use filetime::FileTime;

    pub fn clear_file(path: impl AsRef<Path>, clear_str: &str) -> std::io::Result<()> {
        let path = path.as_ref();

        let md = std::fs::metadata(path)?;
        // let ctime = FileTime::from_system_time(md.created()?);
        let mtime = FileTime::from_system_time(md.modified()?);
        let atime = FileTime::from_system_time(md.accessed()?);

        {
            let mut f = std::fs::File::create(path)?;
            if !clear_str.is_empty() {
                write!(f, "{clear_str}")?;
            }
        }

        filetime::set_file_times(path, atime, mtime)?;

        Ok(())
    }
}
