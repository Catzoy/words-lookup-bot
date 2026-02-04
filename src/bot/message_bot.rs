use crate::bloc::help::HelpBot;
use crate::bloc::phrase_lookup::PhraseLookupBot;
use crate::bloc::start::StartBot;
use crate::bloc::teapot::TeapotBot;
use crate::bloc::thesaurus_lookup::ThesaurusLookupBot;
use crate::bloc::unknown::UnknownBot;
use crate::bloc::urban_lookup::UrbanLookupBot;
use crate::bloc::word_finder::WordFinderBot;
use crate::bloc::word_lookup::WordLookupBot;
use crate::bloc::wordle::WordleBot;
use crate::bot::LookupBot;
use crate::commands::{FullMessageFormatter, MessageCommands};
use crate::format::ToEscaped;
use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{Message, ParseMode};
use teloxide::utils::command::BotCommands;

#[derive(Debug, Clone)]
pub struct MessageBot {
    pub bot: Bot,
    pub message: Message,
}

impl LookupBot for MessageBot {
    type Request = Message;
    type Formatter = FullMessageFormatter;
    type Response = String;

    /// Produces a short, polite error message to present when a query cannot be processed.
    ///
    /// # Returns
    ///
    /// A `String` containing a brief apology and a request to try again later.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = MessageBot::error_response();
    /// assert!(msg.contains("error processing your query"));
    /// ```
    fn error_response() -> Self::Response {
        "There was an error processing your query, try again later, sorry."
            .to_string()
            .to_escaped()
    }

    /// Sends the given text as a message to the chat referenced by this instance's `message`, using MarkdownV2 parsing.
    ///
    /// The message is delivered to `self.message.chat.id` with `ParseMode::MarkdownV2`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if sending the message fails; the error is propagated from the underlying send operation.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example_usage(bot: &crate::bot::MessageBot) -> anyhow::Result<()> {
    /// bot.answer("Hello, world!".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn answer(&self, text: String) -> anyhow::Result<()> {
        let _ = self
            .bot
            .send_message(self.message.chat.id, text)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }
}

impl StartBot<String> for MessageBot {
    /// Provide the welcome message shown when the bot starts.
    ///
    /// The returned string contains a short greeting and brief instructions for using the bot.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // assuming `bot: MessageBot` is available
    /// let msg = bot.start_response();
    /// assert!(msg.starts_with("Hi!"));
    /// ```
    fn start_response(&self) -> String {
        "Hi!\n\
        I'm a bot that can look up words and phrases.\n\
        Simply send me a message and I'll search for the definition of the text."
            .to_string()
            .to_escaped()
    }
}

impl HelpBot<String> for MessageBot {
    /// Provides help text listing the bot's available commands, escaped for MarkdownV2.
    ///
    /// The returned string contains the descriptions of all commands formatted and escaped so it can be sent safely as a MarkdownV2 message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Produces the same escaped help text that MessageBot::help() returns.
    /// let help_text = MessageCommands::descriptions().to_string().to_escaped();
    /// assert!(help_text.contains("help"));
    /// ```
    fn help(&self) -> String {
        MessageCommands::descriptions().to_string().to_escaped()
    }
}

impl TeapotBot<String> for MessageBot {
    /// Returns the HTTP 418 "I'm a teapot" message escaped for MarkdownV2.
    ///
    /// The returned `String` contains the canonical teapot Easter egg text ("I'm a teapot")
    /// with characters escaped for safe use with MarkdownV2 parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = "I'm a teapot".to_string();
    /// assert_eq!(msg, "I'm a teapot");
    /// ```
    fn teapot(&self) -> String {
        "I'm a teapot".to_string().to_escaped()
    }
}

impl UnknownBot<String> for MessageBot {
    /// Provide a polite message indicating the invoked command is not recognized.
    ///
    /// # Returns
    ///
    /// The user-facing reply `"I don't know that command, sorry."` escaped for MarkdownV2.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `mb` is a `MessageBot`:
    /// assert_eq!(mb.unknown_response(), "I don't know that command, sorry.".to_string().to_escaped());
    /// ```
    fn unknown_response(&self) -> String {
        "I don't know that command, sorry.".to_string().to_escaped()
    }
}

impl WordleBot<String> for MessageBot {
    /// Returns a user-facing message indicating today's Wordle could not be retrieved and suggests trying again later.
    /// The returned string is escaped for MarkdownV2 and safe to send directly to users.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = wordle_error_response();
    /// assert!(msg.contains("Could not get today's wordle"));
    /// ```
    fn wordle_error_response() -> String {
        "Could not get today's wordle, sorry, try again in an hour or so."
            .to_string()
            .to_escaped()
    }
}
impl WordLookupBot<String> for MessageBot {
    /// Guidance shown when the user does not provide a word to look up.
    ///
    /// The returned string instructs the user to supply a word (for example: `\word give`).
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_empty();
    /// assert!(msg.contains("specify a word"));
    /// ```
    fn on_empty() -> String {
        "You need to specify a word to look up, like so: `\\word give`"
            .to_string()
            .to_escaped()
    }
}

impl PhraseLookupBot<String> for MessageBot {
    /// Instructs the user to provide a phrase and shows a sample invocation.
    ///
    /// The returned message is escaped for MarkdownV2.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_empty();
    /// assert!(msg.contains("phrase to look up"));
    /// ```
    fn on_empty() -> String {
        "You need to specify a phrase to look up, like so: `\\phrase buckle up`"
            .to_string()
            .to_escaped()
    }
}

impl ThesaurusLookupBot<String> for MessageBot {
    /// Provides guidance instructing the user to supply a phrase for a thesaurus lookup.
    ///
    /// The message includes an example command showing how to invoke the thesaurus.
    ///
    /// # Returns
    ///
    /// `String` containing the guidance message with an example command, escaped for MarkdownV2.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_empty();
    /// assert!(msg.contains("thesaurus"));
    /// ```
    fn on_empty() -> String {
        "You need to specify a phrase to look up, like so: `\\thesaurus cool down`"
            .to_string()
            .to_escaped()
    }
}

impl UrbanLookupBot<String> for MessageBot {
    /// Provide a MarkdownV2-escaped hint showing how to specify a phrase for the Urban lookup command.
    ///
    /// # Returns
    ///
    /// The hint string escaped for MarkdownV2, suggesting a sample command like `\urban gone lemon`.
    ///
    /// # Examples
    ///
    /// ```
    /// let hint = MessageBot::on_empty();
    /// assert!(hint.contains("\\urban gone lemon"));
    /// ```
    fn on_empty() -> String {
        "You need to specify a phrase to look up, like so: `\\urban gone lemon`"
            .to_string()
            .to_escaped()
    }
}

impl WordFinderBot<String> for MessageBot {
    /// Guidance message shown when a finder query is issued without a mask.
    ///
    /// Instructs the user to provide a mask and gives a concrete usage example.
    ///
    /// # Returns
    ///
    /// A `String` containing the guidance text and example mask.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = MessageBot::on_empty();
    /// assert_eq!(msg, "You need to specify a mask to run query for, like so: `\\\\finder a___e`");
    /// ```
    fn on_empty() -> String {
        "You need to specify a mask to run query for, like so: `\\finder a___e`"
            .to_string()
            .to_escaped()
    }

    /// Message shown when a finder query uses an invalid number of symbols.
    ///
    /// Returns a `String` explaining that the finder accepts between two and fifteen symbols.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(
    ///     crate::on_length_invalid(),
    ///     "Sorry, finder can only process up to 15 symbols, but at least two".to_string()
    /// );
    /// ```
    fn on_length_invalid() -> String {
        "Sorry, finder can only process up to 15 symbols, but at least two"
            .to_string()
            .to_escaped()
    }

    /// Describes the required format for finder queries.
    ///
    /// The message states that the query may contain letters (`a-Z`) and underscores with a
    /// maximum length of 15 characters, and that the banned-list may contain only letters with a
    /// maximum length of 13 characters.
    ///
    /// # Returns
    ///
    /// `String` containing the user-facing error message describing allowed characters and lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_wrong_format();
    /// assert!(msg.contains("query"));
    /// assert!(msg.contains("banned"));
    /// ```
    fn on_wrong_format() -> String {
        "Sorry, your message is in the wrong format, you can only specify:\
        1. a-Z and an underscore characters for query, up to 15 chars;\
        2. a-Z characters for banned list, up to 13 chars"
            .to_string()
            .to_escaped()
    }

    /// Explains why a finder query is invalid.
    ///
    /// The message covers two invalid forms: a query made entirely of underscores
    /// (would match the whole dictionary) and a query containing no underscore
    /// (already a complete word).
    ///
    /// # Returns
    ///
    /// `String` describing the invalid query reason.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_invalid_query();
    /// assert!(msg.contains("all underscores") && msg.contains("no underscore"));
    /// ```
    fn on_invalid_query() -> String {
        "Your query is incorrect: \
        it either has all underscores, which would result in a whole dictionary of response, \
        or no underscore, in which case you already know the word!"
            .to_string()
            .to_escaped()
    }
}