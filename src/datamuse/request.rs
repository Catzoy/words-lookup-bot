use crate::datamuse::responses::Word;
use rustify_derive::Endpoint;

#[derive(Endpoint)]
#[endpoint(path = "/words", response = "Vec<Word>")]
pub struct FindWordByMaskRequest {
    #[endpoint(query)]
    sp: String, // mask
}

impl FindWordByMaskRequest {
    /// Creates a `FindWordByMaskRequest` from a mask, converting underscore wildcards to `?` to match Datamuse's mask syntax.
    ///
    /// The provided `mask` may use `_` to indicate a single-character wildcard; this constructor replaces all `_` with `?` and stores the result in the request's `sp` query field.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = FindWordByMaskRequest::new("t_st".to_string());
    /// assert_eq!(req.sp, "t?st");
    /// ```
    pub fn new(mask: String) -> Self {
        Self {
            sp: mask.replace("_", "?"),
        }
    }
}