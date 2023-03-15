#[cfg(test)]
mod tests;

pub use fns::{clear_file, change_file_content}; 
pub use fns::{clear_dir_files, change_dir_files_content};

pub struct FileRegex {
    white_list_regex: Option<regex::Regex>,
    black_list_regex: Option<regex::Regex>,
}
impl FileRegex {
    pub const EMPTY: Self = Self{ white_list_regex: None, black_list_regex: None };

    pub fn new_regex(white: Option<regex::Regex>, black: Option<regex::Regex>) -> Self {
        Self {
            white_list_regex: white,
            black_list_regex: black,
        }
    }
    pub fn new(white: Option<&str>, black: Option<&str>) -> Result<Self, regex::Error> {
        let white_list_regex = match white {
            Some(re) => Some(regex::Regex::new(re)?),
            _ => None,
        };
        let black_list_regex = match black {
            Some(re) => Some(regex::Regex::new(re)?),
            _ => None,
        };

        Ok(Self {
            white_list_regex,
            black_list_regex,
        })
    }

    pub fn is_valid(&self, path: impl AsRef<std::path::Path>) -> bool {
        if let Some(path) = path.as_ref().to_str() {
            let white_ok = self.white_list_regex.as_ref()
                .map(|wl_re|wl_re.is_match(path)).unwrap_or(true);
            let black_ok = self.black_list_regex.as_ref()
                .map(|bl_re|!bl_re.is_match(path)).unwrap_or(true);
            white_ok && black_ok
        } else {
            // if there exist white list and path is not UTF-8 => path IS NOT matched to whitelist
            let white_ok = self.white_list_regex.is_none();
            // if there exist white list and path is not UTF-8 => path IS matched to blacklist
            let black_ok = true;
            white_ok && black_ok
        }
    }
}

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
