
/// white/black list filter
pub struct WbFilter {
    white_list_regex: Option<regex::Regex>,
    black_list_regex: Option<regex::Regex>,
}

impl WbFilter {
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
    /// # return
    /// * `true` if the `text` allowed
    /// * `false` if the `text` denied
    pub fn is_allowed(&self, text: &str) -> bool {
        let white_ok = self.white_list_regex.as_ref()
            .map(|wl_re|wl_re.is_match(text)).unwrap_or(true);
        let black_ok = self.black_list_regex.as_ref()
            .map(|bl_re|!bl_re.is_match(text)).unwrap_or(true);
        white_ok && black_ok
    }

    /// opposite to `fn is_denied_opt`
    /// # params
    /// * `text`:
    ///   + `Some(..)` => `text` is UTF-8
    ///   + `None` => `text` is not UTF-8
    /// # return
    /// * `true` if the `text` allowed
    /// * `false` if the `text` denied
    pub fn is_allowed_opt(&self, text: Option<&str>) -> bool {
        if let Some(text) = text {
            self.is_allowed(text)
        } else {
            // if there exist white list and path is not UTF-8 => path IS NOT matched to whitelist => deny
            let white_ok = self.white_list_regex.is_none();
            // if there exist black list and path is not UTF-8 => path IS NOT matched to blacklist => allow
            let black_ok = true;
            white_ok && black_ok
        }
    }
    
    /// opposite to `fn is_allowed`
    /// # return
    /// * `true` if the `text` denied
    /// * `false` if the `text` allowed
    pub fn is_denied(&self, text: &str) -> bool {
        !self.is_allowed(text)
    }
        
    /// opposite to `fn is_allowed_opt`
    /// # params
    /// * `text`:
    ///   + `Some(..)` => `text` is UTF-8
    ///   + `None` => `text` is not UTF-8
    /// # return
    /// * `true` if the `text` denied
    /// * `false` if the `text` allowed
    pub fn is_denied_opt(&self, text: Option<&str>) -> bool {
        !self.is_allowed_opt(text)
    }
}