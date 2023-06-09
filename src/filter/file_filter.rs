use super::wb_filter::WbFilter;

/// white/black list file filter
pub struct FileFilter(WbFilter);

impl FileFilter {
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
    /// **\[!\]** this `fn` checks that the path exists and it is a file
    /// # return
    /// * `Some(true)` if the file allowed
    /// * `Some(false)` if the file denied
    /// * `None` if there no file name in the path or the path is not a file
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
        Some(self.0.is_allowed_opt(path.to_str()))
    }

    
    /// opposite to `fn is_allowed`
    /// 
    /// **\[!\]** this `fn` checks that the path exists and is a file
    /// # return
    /// * `Some(true)` if the file denied
    /// * `Some(false)` if the file allowed
    /// * `None` if there no file name in the path or the path is not a file
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

#[cfg(test)]
mod test {
    use super::FileFilter;
    
    const NON_EX_PATH_01: &str = "./tests/please/dont/create/such.file";
    const NON_EX_PATH_02: &str = "./tests/please/dont/create/tmp_test_ff_99.txt";
    const NON_EX_PATH_03: &str = "./tests/please/dont/create/tmp_test_ff_99_v_02.txt";
    const NON_EX_PATHS: &[&str] = &[NON_EX_PATH_01, NON_EX_PATH_02, NON_EX_PATH_03];

    const EX_PATH_01: &str = "./tests/tmp_test_ff_01.txt";
    const EX_PATH_02: &str = "./tests/tmp_test_ff_02.txt";
    const EX_PATH_03: &str = "./tests/tmp_test_ff_03.lib";
    const EX_PATH_04: &str = "./tests/tmp_test_ff_04.txt";
    const EX_PATH_05: &str = "./tests/tmp_xtest_ff_05.txt";
    const EX_PATHS: &[&str] = &[EX_PATH_01, EX_PATH_02, EX_PATH_03, EX_PATH_04, EX_PATH_05];
    
    
    fn test_prepare() {
        static PREPARED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        
        let prepared = PREPARED.swap(true, std::sync::atomic::Ordering::AcqRel);
        if prepared { return }

        for path in NON_EX_PATHS {
            let _ = std::fs::remove_file(path);
        }

        std::fs::create_dir_all("./tests").unwrap();
        for path in EX_PATHS {
            let _ = std::fs::File::create(path).unwrap(); // immediately drop
        }
    }


    #[test]
    fn test_ff_01_empty() {
        test_prepare();
        let filter = FileFilter::new(None, None).unwrap();
        
        assert_eq!(Some(true), filter.is_allowed_unchecked(EX_PATH_01));
        assert_eq!(Some(false), filter.is_denied_unchecked(EX_PATH_01));
        assert_eq!(Some(true), filter.is_allowed(EX_PATH_01));
        assert_eq!(Some(false), filter.is_denied(EX_PATH_01));

        assert_eq!(Some(true), filter.is_allowed_unchecked(NON_EX_PATH_01));
        assert_eq!(Some(false), filter.is_denied_unchecked(NON_EX_PATH_01));
        assert_eq!(None, filter.is_allowed(NON_EX_PATH_01));
        assert_eq!(None, filter.is_denied(NON_EX_PATH_01));
    }

    #[test]
    fn test_ff_02_whitelist() {
        test_prepare();
        let filter = FileFilter::new(Some(r"tmp_test_.*\.txt"), None).unwrap();
        
        let non_ex_allow = [false, true, true];
        let ex_allow = [true, true, false, true, false];

        for (i, non_ex_path) in NON_EX_PATHS.iter().enumerate() {
            assert_eq!(Some(non_ex_allow[i]), filter.is_allowed_unchecked(non_ex_path));
            assert_eq!(Some(!non_ex_allow[i]), filter.is_denied_unchecked(non_ex_path));
            assert_eq!(None, filter.is_allowed(non_ex_path));
            assert_eq!(None, filter.is_denied(non_ex_path));    
        }

        for (i, non_ex_path) in EX_PATHS.iter().enumerate() {
            assert_eq!(Some(ex_allow[i]), filter.is_allowed_unchecked(non_ex_path));
            assert_eq!(Some(!ex_allow[i]), filter.is_denied_unchecked(non_ex_path));
            assert_eq!(Some(ex_allow[i]), filter.is_allowed(non_ex_path));
            assert_eq!(Some(!ex_allow[i]), filter.is_denied(non_ex_path));    
        }
    }

    #[test]
    fn test_ff_03_blacklist() {
        test_prepare();
        let filter = FileFilter::new(None, Some(r"tmp_test_.*\.txt")).unwrap();
        
        let non_ex_allow = [true, false, false];
        let ex_allow = [false, false, true, false, true];

        for (i, non_ex_path) in NON_EX_PATHS.iter().enumerate() {
            assert_eq!(Some(non_ex_allow[i]), filter.is_allowed_unchecked(non_ex_path));
            assert_eq!(Some(!non_ex_allow[i]), filter.is_denied_unchecked(non_ex_path));
            assert_eq!(None, filter.is_allowed(non_ex_path));
            assert_eq!(None, filter.is_denied(non_ex_path));    
        }

        for (i, non_ex_path) in EX_PATHS.iter().enumerate() {
            assert_eq!(Some(ex_allow[i]), filter.is_allowed_unchecked(non_ex_path));
            assert_eq!(Some(!ex_allow[i]), filter.is_denied_unchecked(non_ex_path));
            assert_eq!(Some(ex_allow[i]), filter.is_allowed(non_ex_path));
            assert_eq!(Some(!ex_allow[i]), filter.is_denied(non_ex_path));    
        }
    }

    #[test]
    fn test_ff_04_wb_list() {
        test_prepare();
        let filter = FileFilter::new(Some(r"tmp_test_.*\.txt"), Some(r".*_02.*")).unwrap();
        
        let non_ex_allow = [false, true, false];
        let ex_allow = [true, false, false, true, false];

        for (i, non_ex_path) in NON_EX_PATHS.iter().enumerate() {
            assert_eq!(Some(non_ex_allow[i]), filter.is_allowed_unchecked(non_ex_path));
            assert_eq!(Some(!non_ex_allow[i]), filter.is_denied_unchecked(non_ex_path));
            assert_eq!(None, filter.is_allowed(non_ex_path));
            assert_eq!(None, filter.is_denied(non_ex_path));    
        }

        for (i, non_ex_path) in EX_PATHS.iter().enumerate() {
            assert_eq!(Some(ex_allow[i]), filter.is_allowed_unchecked(non_ex_path));
            assert_eq!(Some(!ex_allow[i]), filter.is_denied_unchecked(non_ex_path));
            assert_eq!(Some(ex_allow[i]), filter.is_allowed(non_ex_path));
            assert_eq!(Some(!ex_allow[i]), filter.is_denied(non_ex_path));    
        }
    }
}
