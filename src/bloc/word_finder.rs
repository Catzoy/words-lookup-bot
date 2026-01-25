use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::{LookupBot, LookupBotX};
use crate::datamuse::client::DatamuseClient;
use crate::datamuse::request::FindWordByMaskRequest;
use crate::format::LookupFormatter;
use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;
use teloxide::dptree::entry;

static WORD_FIND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^([a-z_]+),? ?([a-z]*)$").unwrap());

pub trait WordFinderBot<Response>
where
    Response: Send + Default,
{
    /// Provides the default response to use when the user submits an empty query.
    ///
    /// By default this returns `Default::default()` for the response type.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Bot;
    /// impl word_finder::WordFinderBot<String> for Bot {}
    /// let resp = Bot::on_empty();
    /// assert_eq!(resp, String::default());
    /// ```
    fn on_empty() -> Response {
        Default::default()
    }

    /// Provides the default response used when the user-supplied mask has an invalid length.
    ///
    /// Returns the default `Response` value.
    ///
    /// # Examples
    ///
    /// ```
    /// struct BotDummy;
    /// impl WordFinderBot<()> for BotDummy {}
    /// let resp = BotDummy::on_length_invalid();
    /// assert_eq!(resp, ());
    /// ```
    fn on_length_invalid() -> Response {
        Default::default()
    }

    /// Produce the user-facing response for masks that contain unsupported characters.
    ///
    /// This hook is called when the provided mask contains characters other than ASCII letters or `_`.
    ///
    /// # Returns
    ///
    /// The response value to send to the user; default implementation returns `Default::default()`.
    ///
    /// # Examples
    ///
    /// ```
    /// let default_resp: String = Default::default();
    /// // `on_wrong_format()` returns the same default value for the bot's Response type.
    /// ```
    fn on_wrong_format() -> Response {
        Default::default()
    }

    /// Produces the response the bot should send when the user's query is invalid.
    ///
    /// The default implementation returns `Default::default()` for the response type.
    /// Implementors may override to provide a custom user-facing reply.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::default::Default;
    /// struct Resp;
    /// impl Default for Resp { fn default() -> Self { Resp } }
    ///
    /// trait WordFinderBot<R> { fn on_invalid_query() -> R where R: Default { Default::default() } }
    ///
    /// struct MyBot;
    /// impl WordFinderBot<Resp> for MyBot {}
    ///
    /// let resp = MyBot::on_invalid_query();
    /// ```
    fn on_invalid_query() -> Response {
        Default::default()
    }
}

pub trait WordFinderHandler {
    /// Retrieve candidate words from Datamuse that match a given `FinderMask`.
    ///
    /// The request uses `mask.mask` as the Datamuse mask (where `'_'` denotes unknown letters),
    /// and the resulting list is filtered to exclude words containing any characters from
    /// `mask.banned`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Requires an async runtime.
    /// // let client = DatamuseClient::new();
    /// // let mask = FinderMask::from("_at, b".into()).unwrap();
    /// // let words = tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// //     get_possible_words(client, mask).await
    /// // })?;
    /// // assert!(words.iter().any(|w| w.ends_with("at")));
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(Vec<String>)` with words matching the mask and not containing banned letters,
    /// or `Err(LookupError::FailedRequest)` if the remote request fails.
    async fn get_possible_words(
        client: DatamuseClient,
        mask: FinderMask,
    ) -> Result<Vec<String>, LookupError> {
        client
            .exec(FindWordByMaskRequest::new(mask.mask.clone()))
            .await
            .map(|vec| mask.retain_only_allowed(vec))
            .map_err(|err| {
                log::error!("WF failed request: {}", err);
                LookupError::FailedRequest
            })
    }

    async fn ensure_valid(&self, mask: String) -> Option<FinderMask>;

    fn word_finder_handler() -> CommandHandler;
}

trait WordFinderFormatter<Value> {
    fn compose_word_finder_response(self, defs: Vec<String>) -> Result<Value, LookupError>;
}

impl<Formatter> WordFinderFormatter<Formatter::Value> for Formatter
where
    Formatter: LookupFormatter,
{
    /// Compose a formatted response value from a list of word definitions.
    ///
    /// Appends a title `"Found N words"` where `N` is the number of provided definitions,
    /// visits each definition in order, and finalizes the formatter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let defs = vec!["apple".to_string(), "angle".to_string()];
    /// let response = Formatter::new().compose_word_finder_response(defs).unwrap();
    /// ```
    ///
    /// # Returns
    ///
    /// The constructed formatter value on success, or `LookupError::FailedResponseBuilder` if building fails.
    fn compose_word_finder_response(
        mut self,
        defs: Vec<String>,
    ) -> Result<Formatter::Value, LookupError> {
        self.append_title(format!("Found {} words", defs.len()));
        for (i, def) in defs.into_iter().enumerate() {
            self.visit_word_finder_definition(i, def);
        }
        self.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinderMask {
    mask: String,
    banned: String,
}

#[derive(Debug, PartialEq)]
enum MaskParsingError {
    WrongFormat,
    InvalidLength,
    InvalidQuery,
}

impl FinderMask {
    /// Parse a user-provided mask string into a `FinderMask`, validating format, lengths, and content.
    ///
    /// The input may include an optional comma-separated banlist (e.g. `"a__ow, jfk"`) or consist of only the mask (`"a__ow"`).
    /// Validations performed:
    /// - Mask length must be between 2 and 15 characters.
    /// - Banlist length must be at most 13 characters.
    /// - Mask must contain at least one underscore (`'_'`) and at least one non-underscore character.
    ///
    /// On success returns a `FinderMask` with `mask` set to the parsed mask and `banned` set to the parsed banlist (or an empty string when absent).
    ///
    /// # Examples
    ///
    /// ```
    /// let fm = FinderMask::from("a__ow, jfk".to_string()).unwrap();
    /// assert_eq!(fm.mask, "a__ow");
    /// assert_eq!(fm.banned, "jfk");
    /// ```
    fn from(mask: String) -> Result<FinderMask, MaskParsingError> {
        let parsed = WORD_FIND
            .captures(&mask)
            .ok_or(MaskParsingError::WrongFormat)?;

        let finder_mask = parsed
            .get(1)
            .map(|m| m.as_str())
            .ok_or(MaskParsingError::WrongFormat)?;
        if finder_mask.len() < 2 || finder_mask.len() > 15 {
            return Err(MaskParsingError::InvalidLength);
        }

        let banned_list = parsed.get(2).map(|m| m.as_str()).unwrap_or("");
        if banned_list.len() > 13 {
            return Err(MaskParsingError::InvalidLength);
        }

        let (has_blank, has_filled) = finder_mask
            .chars()
            .fold((false, false), |(has_blank, has_filled), char| {
                (has_blank || char == '_', has_filled || char != '_')
            });
        if !has_blank || !has_filled {
            return Err(MaskParsingError::InvalidQuery);
        }

        let combo = FinderMask {
            mask: finder_mask.to_string(),
            banned: banned_list.to_string(),
        };
        Ok(combo)
    }

    /// Filter candidate words by removing any that contain characters from `self.banned`.
    ///
    /// The method returns a new vector containing only the input words that do not include any banned character.
    /// If the input vector is empty or the banlist cannot be compiled into a regex, the original vector is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// let mask = FinderMask { mask: "a__".into(), banned: "bd".into() };
    /// let words = vec!["cat".to_string(), "dog".to_string(), "bat".to_string()];
    /// let filtered = mask.retain_only_allowed(words);
    /// assert_eq!(filtered, vec!["cat".to_string()]);
    /// ```
    fn retain_only_allowed(&self, vec: Vec<String>) -> Vec<String> {
        if vec.is_empty() {
            return vec;
        }

        let mut banned = HashSet::new();
        for char in self.banned.chars() {
            banned.insert(char);
        }

        let banned = banned.into_iter().collect::<String>();
        match Regex::new(format!("[{}]", banned.as_str()).as_str()).ok() {
            Some(bans) => vec
                .into_iter()
                .filter(|it| bans.find(it).is_none())
                .collect(),
            None => vec,
        }
    }
}

impl<Bot, Formatter> WordFinderHandler for Bot
where
    Bot: WordFinderBot<Bot::Response> + LookupBot<Formatter = Formatter> + Send + Sync + 'static,
    Formatter: LookupFormatter<Value = Bot::Response>,
{
    /// Validate a mask string and send a user-facing response when it is invalid.
    ///
    /// Parses the provided mask into a `FinderMask` and returns `Some(FinderMask)` on success.
    /// If parsing fails, sends the corresponding response (`on_wrong_format`, `on_length_invalid`,
    /// or `on_invalid_query`) and returns `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # async fn run<B: crate::bloc::word_finder::WordFinderBot<B::Response> + Send + Sync + 'static>(bot: &B) {
    /// let maybe_mask = bot.ensure_valid("a__le, xyz".to_string()).await;
    /// match maybe_mask {
    ///     Some(mask) => { /* use mask */ }
    ///     None => { /* user was notified about the error */ }
    /// }
    /// # }
    /// ```
    async fn ensure_valid(&self, mask: String) -> Option<FinderMask> {
        match FinderMask::from(mask) {
            Ok(it) => Some(it),
            Err(err) => {
                let response = match err {
                    MaskParsingError::WrongFormat => Self::on_wrong_format(),
                    MaskParsingError::InvalidLength => Self::on_length_invalid(),
                    MaskParsingError::InvalidQuery => Self::on_invalid_query(),
                };
                let _ = self.answer(response).await;
                None
            }
        }
    }
    /// Constructs the Teloxide command handler for the word-finder feature.
    ///
    /// The handler validates a user-provided mask, performs a lookup for matching words,
    /// filters results according to any banned letters, formats the response, and sends it
    /// back to the user.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct the handler; run-time wiring (bot, dispatcher) is required to execute it.
    /// let _handler: CommandHandler = word_finder_handler();
    /// ```
    fn word_finder_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, mask: String| async move {
                bot.drop_empty(mask, Self::on_empty).await
            })
            .filter_map_async(|bot: Bot, mask: String| async move { bot.ensure_valid(mask).await })
            .map_async(Self::get_possible_words)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<String>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(move |bot: Bot, defs: Vec<String>| {
                bot.formatter().compose_word_finder_response(defs)
            })
            .filter_map_async(
                |bot: Bot, response: Result<Bot::Response, LookupError>| async move {
                    bot.retrieve_or_generic_err(response).await
                },
            )
            .endpoint(
                |bot: Bot, response: Bot::Response| async move { bot.respond(response).await },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finder_mask_disallows_small() {
        let input = String::from("a");
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::InvalidLength));
    }
    #[test]
    fn finder_mask_disallows_big() {
        let input = String::from("a").repeat(20);
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::InvalidLength));
    }
    #[test]
    fn finder_mask_disallows_empty() {
        let input = String::from("");
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::WrongFormat));
    }
    #[test]
    fn finder_mask_disallows_unknown_chars() {
        let input = String::from("123");
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::WrongFormat));
    }
    #[test]
    fn finder_mask_disallows_wrong_format() {
        let input = String::from("abc, abc, abc");
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::WrongFormat));
    }
    #[test]
    fn finder_mask_disallows_long_banlist() {
        let input = String::from("abc, abcdefghijklmnopqrstuvwxyz");
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::InvalidLength));
    }
    #[test]
    fn finder_mask_disallows_wrong_mask_format_all_letters() {
        let input = String::from("abc, jfk");
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::InvalidQuery));
    }
    #[test]
    fn finder_mask_disallows_wrong_mask_format_all_underscores() {
        let input = String::from("___, jfk");
        let output = FinderMask::from(input);
        assert_eq!(output, Err(MaskParsingError::InvalidQuery));
    }
    #[test]
    fn finder_mask_allows_only_mask() {
        let input = String::from("a__ow");
        let output = FinderMask::from(input);
        assert_eq!(
            output,
            Ok(FinderMask {
                mask: String::from("a__ow"),
                banned: String::from(""),
            })
        );
    }
    #[test]
    fn finder_mask_allows_only_mask_partial_banlist() {
        let input = String::from("a__ow,");
        let output = FinderMask::from(input);
        assert_eq!(
            output,
            Ok(FinderMask {
                mask: String::from("a__ow"),
                banned: String::from(""),
            })
        );
    }
    #[test]
    fn finder_mask_allows_only_mask_with_banlist() {
        let input = String::from("a__ow, jfk");
        let output = FinderMask::from(input);
        assert_eq!(
            output,
            Ok(FinderMask {
                mask: String::from("a__ow"),
                banned: String::from("jfk"),
            })
        );
    }

    #[test]
    fn finder_mask_retains_empty() {
        let mask = FinderMask {
            mask: String::from(""),
            banned: String::from(""),
        };
        let words = Vec::<String>::new();
        let retained = mask.retain_only_allowed(words.clone());
        assert_eq!(retained, words);
    }
    /// Asserts that no words are removed when the ban list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let mask = FinderMask { mask: String::from(""), banned: String::from("") };
    /// let words = vec![String::from("abra"), String::from("cadabra")];
    /// let retained = mask.retain_only_allowed(words.clone());
    /// assert_eq!(retained, words);
    /// ```
    #[test]
    fn finder_mask_retains_every_on_empty_banlist() {
        let mask = FinderMask {
            mask: String::from(""),
            banned: String::from(""),
        };
        let words = vec![String::from("abra"), String::from("cadabra")];
        let retained = mask.retain_only_allowed(words.clone());
        assert_eq!(retained, words);
    }
    #[test]
    fn finder_mask_retains_none_on_full_banlist_match() {
        let mask = FinderMask {
            mask: String::from(""),
            banned: String::from("abcdr"),
        };
        let words = vec![String::from("abra"), String::from("cadabra")];
        let retained = mask.retain_only_allowed(words);
        assert_eq!(retained, Vec::<String>::new());
    }
    #[test]
    fn finder_mask_retains_some_on_partial_banlist_match() {
        let mask = FinderMask {
            mask: String::from(""),
            banned: String::from("abcdr"),
        };
        let words = vec![
            String::from("abra"),
            String::from("cadabra"),
            String::from("poke"),
        ];
        let retained = mask.retain_only_allowed(words);
        assert_eq!(retained, vec![String::from("poke")]);
    }
    #[test]
    fn finder_mask_retains_all_on_no_banlist_match() {
        let mask = FinderMask {
            mask: String::from(""),
            banned: String::from("wqf"),
        };
        let words = vec![
            String::from("abra"),
            String::from("cadabra"),
            String::from("poke"),
        ];
        let retained = mask.retain_only_allowed(words.clone());
        assert_eq!(retained, words);
    }
    #[test]
    fn finder_mask_retains_same_with_duplicates() {
        let mask1 = FinderMask {
            mask: String::from(""),
            banned: String::from("abc"),
        };
        let mask2 = FinderMask {
            mask: String::from(""),
            banned: String::from("abcabc"),
        };
        let words = vec![
            String::from("abra"),
            String::from("cadabra"),
            String::from("poke"),
        ];
        let retained1 = mask1.retain_only_allowed(words.clone());
        let retained2 = mask2.retain_only_allowed(words.clone());
        assert_eq!(retained1, vec![String::from("poke")]);
        assert_eq!(retained2, retained1);
    }
}
