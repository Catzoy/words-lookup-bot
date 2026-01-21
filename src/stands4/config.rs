#[derive(Clone)]
pub struct Stands4Config {
    pub(crate) user_id: String,
    pub(crate) token: String,
    pub(crate) format: String,
}

impl Stands4Config {
    pub fn new(user_id: String, token: String) -> Self {
        Stands4Config {
            user_id,
            token,
            format: "json".to_string(),
        }
    }
}
