use super::wb_filter::WbFilter;

/// white/black list dir filter
pub struct DirFilter(WbFilter);

impl DirFilter {
    pub const EMPTY: Self = Self(WbFilter::EMPTY);

    pub fn new_filter(filter: WbFilter) -> Self {
        Self(filter)
    }
    pub fn new_regex(white: Option<regex::Regex>, black: Option<regex::Regex>) -> Self {
        Self(WbFilter::new_regex(white, black))
    }
    pub fn new(white: Option<&str>, black: Option<&str>) -> Result<Self, regex::Error> {
        Ok(Self(WbFilter::new(white, black)?))
    }

    /// opposite to `fn is_denied`
    /// 
    /// **\[!\]** this `fn` checks that the path exists and it is a dir
    /// # return
    /// * `Some(true)` if the dir allowed
    /// * `Some(false)` if the dir denied
    /// * `None` if the path is not exist or not a dir
    pub fn is_allowed(&self, path: impl AsRef<std::path::Path>) -> Option<bool> {
        let path = path.as_ref();
        if path.is_dir() {
            Some(self.is_allowed_unchecked(path))
        } else {
            None
        }
    }

    /// `unchecked` postfix means that we dont actually test that the path is exists and it is a dir  
    /// 
    /// opposite to `fn is_denied_unchecked`
    /// # return
    /// * `true` if the dir allowed
    /// * `false` if the dir denied
    pub fn is_allowed_unchecked(&self, path: impl AsRef<std::path::Path>) -> bool {
        let path = path.as_ref();
        let path = path_slash::PathExt::to_slash(path);
        let path = path.as_ref().map(|x|x.as_ref());
        self.0.is_allowed_opt(path)
    }

    
    /// opposite to `fn is_allowed`
    /// 
    /// **\[!\]** this `fn` checks that the path exists and is a dir
    /// # return
    /// * `Some(true)` if the dir denied
    /// * `Some(false)` if the dir allowed
    /// * `None` if the path is not exist or not a dir
    pub fn is_denied(&self, path: impl AsRef<std::path::Path>) -> Option<bool> {
        self.is_allowed(path).map(|x|!x)
    }

    /// `unchecked` postfix means that we dont actually test that the path is exists and it is a dir  
    /// 
    /// opposite to `fn is_allowed_unchecked`
    /// # return
    /// * `true` if the dir denied
    /// * `false` if the dir allowed
    /// * `None` if the path is empty
    pub fn is_denied_unchecked(&self, path: impl AsRef<std::path::Path>) -> bool {
        !self.is_allowed_unchecked(path)
    }
}
