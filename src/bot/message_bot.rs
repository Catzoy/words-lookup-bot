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
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{Message, ParseMode};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

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
        "There was an error processing your query, try again later, sorry.".to_string()
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
    /// Provide the standard teapot reply used as the HTTP 418 Easter egg.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = "I'm a teapot".to_string();
    /// assert_eq!(msg, "I'm a teapot");
    /// ```
    fn teapot(&self) -> String {
        "I'm a teapot".to_string()
    }
}

impl UnknownBot<String> for MessageBot {
    /// Indicates that the invoked command is not recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// // The bot should reply with this message when a command is unknown.
    /// assert_eq!( "I don't know that command, sorry.", "I don't know that command, sorry.");
    /// ```
    fn unknown_response(&self) -> String {
        "I don't know that command, sorry.".to_string()
    }
}

impl WordleBot<String> for MessageBot {
    /// Provides a user-facing message indicating today's Wordle could not be retrieved and suggests trying again later.
    ///
    /// The returned string is suitable for sending directly to users.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = wordle_error_response();
    /// assert!(msg.contains("Could not get today's wordle"));
    /// ```
    fn wordle_error_response() -> String {
        "Could not get today's wordle, sorry, try again in an hour or so.".to_string()
    }
}
impl WordLookupBot<String> for MessageBot {
    /// Provides guidance when no word is supplied for lookup.
    ///
    /// Returns the guidance string instructing the user how to specify a word (example: `\word give`).
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_empty();
    /// assert_eq!(msg, "You need to specify a word to look up, like so: `\\word give`");
    /// ```
    fn on_empty() -> String {
        "You need to specify a word to look up, like so: `\\word give`".to_string()
    }
}

impl PhraseLookupBot<String> for MessageBot {
    /// Provides a user-facing message instructing the user to specify a phrase to look up.
    ///
    /// The message includes an example invocation showing the `\phrase` command.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_empty();
    /// assert_eq!(msg, "You need to specify a phrase to look up, like so: `\\phrase buckle up`");
    /// ```
    fn on_empty() -> String {
        "You need to specify a phrase to look up, like so: `\\phrase buckle up`".to_string()
    }
}

impl ThesaurusLookupBot<String> for MessageBot {
    /// Provides a guidance message instructing the user to supply a phrase for a thesaurus lookup.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_empty();
    /// assert_eq!(msg, "You need to specify a phrase to look up, like so: `\\thesaurus cool down`");
    /// ```
    fn on_empty() -> String {
        "You need to specify a phrase to look up, like so: `\\thesaurus cool down`".to_string()
    }
}

impl UrbanLookupBot<String> for MessageBot {
    /// Provide a user-facing hint instructing how to specify a phrase for the Urban lookup command.
    ///
    /// # Returns
    ///
    /// A message suggesting how to specify a phrase, for example `\urban gone lemon`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example usage — adjust path if `MessageBot` is in a different module.
    /// let hint = MessageBot::on_empty();
    /// assert!(hint.contains("\\urban gone lemon"));
    /// ```
    fn on_empty() -> String {
        "You need to specify a phrase to look up, like so: `\\urban gone lemon`".to_string()
    }
}

impl WordFinderBot<String> for MessageBot {
    /// Returns the guidance message shown when a word-finder query is issued without a mask.
    ///
    /// The returned string instructs the user to provide a mask and gives a concrete example.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = MessageBot::on_empty();
    /// assert_eq!(msg, "You need to specify a mask to run query for, like so: `\\\\finder a___e`");
    /// ```
    fn on_empty() -> String {
        "You need to specify a mask to run query for, like so: `\\finder a___e`".to_string()
    }

    /// Provide the error message used when a finder query has an invalid length.
    ///
    /// # Returns
    ///
    /// A `String` explaining that the finder accepts between two and 15 symbols.
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
        "Sorry, finder can only process up to 15 symbols, but at least two".to_string()
    }

    /// Provide an error message for queries containing characters outside `a-z`, `A-Z`, or underscore.
    ///
    /// # Returns
    ///
    /// `String` with an error message indicating only `a-z`, `A-Z`, and underscore are permitted.
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = on_unknown_character();
    /// assert_eq!(
    ///     msg,
    ///     "Sorry, your message contains unsupported characters - only a-z, A-Z and an underscore can be specified".to_string()
    /// );
    /// ```
    fn on_unknown_character() -> String {
        "Sorry, your message contains unsupported characters - only a-z, A-Z and an underscore can be specified".to_string()
    }

    /// Return an error message describing why a word-finder query is invalid.
    ///
    /// The message explains two invalid forms: a query made entirely of underscores (would match the whole dictionary)
    /// or a query containing no underscore (already a complete word).
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
    }
}
