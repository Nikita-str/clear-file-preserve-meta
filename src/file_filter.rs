
pub struct FileFilter {
    white_list_regex: Option<regex::Regex>,
    black_list_regex: Option<regex::Regex>,
}
impl FileFilter {
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

    /// opposite to `fn is_denied`
    /// 
    /// **\[!\]** this `fn` checks that the path exists and is a file
    /// # return
    /// * `Some(true)` if the file allowed
    /// * `Some(false)` if the file denied
    /// * `None` if there no file name in the path
    pub fn is_allowed(&self, path: impl AsRef<std::path::Path>) -> Option<bool> {
        let path = path.as_ref();
        if path.is_file() {
            self.is_allowed_unchecked(path)
        } else {
            None
        }
    }

    /// `unchecked` postfix means that we dont actually test that the path is exists and it is a file  
    /// 
    /// opposite to `fn is_denied_unchecked`
    /// # return
    /// * `Some(true)` if the file allowed
    /// * `Some(false)` if the file denied
    /// * `None` if there no file name in the path
    pub fn is_allowed_unchecked(&self, path: impl AsRef<std::path::Path>) -> Option<bool> {
        let Some(path) = path.as_ref().file_name() else { return None };

        if let Some(path) = path.to_str() {
            let white_ok = self.white_list_regex.as_ref()
                .map(|wl_re|wl_re.is_match(path)).unwrap_or(true);
            let black_ok = self.black_list_regex.as_ref()
                .map(|bl_re|!bl_re.is_match(path)).unwrap_or(true);
            Some(white_ok && black_ok)
        } else {
            // if there exist white list and path is not UTF-8 => path IS NOT matched to whitelist
            let white_ok = self.white_list_regex.is_none();
            // if there exist white list and path is not UTF-8 => path IS matched to blacklist
            let black_ok = true;
            Some(white_ok && black_ok)
        }
    }

    
    /// opposite to `fn is_denied`
    /// 
    /// **\[!\]** this `fn` checks that the path exists and is a file
    /// # return
    /// * `Some(true)` if the file denied
    /// * `Some(false)` if the file allowed
    /// * `None` if there no file name in the path
    pub fn is_denied(&self, path: impl AsRef<std::path::Path>) -> Option<bool> {
        self.is_allowed(path).map(|x|!x)
    }

    /// `unchecked` postfix means that we dont actually test that the path is exists and it is a file  
    /// 
    /// opposite to `fn is_allowed_unchecked`
    /// # return
    /// * `Some(true)` if the file denied
    /// * `Some(false)` if the file allowed
    /// * `None` if there no file name in the path
    pub fn is_denied_unchecked(&self, path: impl AsRef<std::path::Path>) -> Option<bool> {
        self.is_allowed_unchecked(path).map(|x|!x)
    }
}
