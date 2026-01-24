#[derive(Clone)]
pub struct Stands4Config {
    pub(crate) user_id: String,
    pub(crate) token: String,
    pub(crate) format: String,
}

impl Stands4Config {
    /// Creates a Stands4Config for the given user ID and token with the `format` field set to `"json"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::stands4::config::Stands4Config;
    ///
    /// let cfg = Stands4Config::new("user123".into(), "tokenABC".into());
    /// ```
    pub fn new(user_id: String, token: String) -> Self {
        Stands4Config {
            user_id,
            token,
            format: "json".to_string(),
        }
    }
}