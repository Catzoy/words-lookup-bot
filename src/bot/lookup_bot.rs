use crate::bloc::common::LookupError;
use crate::format::LookupFormatter;
use shuttle_runtime::async_trait;

#[async_trait]
pub trait LookupBot: Clone {
    type Request: Clone + Send + Sync;
    type Formatter: LookupFormatter + Default;
    type Response: Clone + Send + Sync + Default;

    /// Returns the default formatter for this bot.
    ///
    /// The formatter is created by calling `Formatter::default()` for the bot's associated
    /// `Formatter` type.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a value `bot` that implements `LookupBot`:
    /// let formatter = bot.formatter();
    /// let default = <_ as LookupBot>::Formatter::default();
    /// // `formatter` is the same as `default` for the bot's Formatter type.
    /// ```
    fn formatter(&self) -> Self::Formatter {
        Self::Formatter::default()
    }

    /// Provides the standard error response for failed lookup operations.
    ///
    /// This returns the default value of `Self::Response`, which implementations treat as the canonical
    /// error payload to send when a lookup cannot be satisfied.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assuming `MyBot` implements `LookupBot`:
    /// // let err = MyBot::error_response();
    /// // assert_eq!(err, MyBot::Response::default());
    /// ```
    fn error_response() -> Self::Response {
        Self::Response::default()
    }

    /// Create a default response used for empty lookup results.
    
    ///
    
    /// This returns the type's `Default` response value to represent an empty or no-op reply.
    
    ///
    
    /// # Examples
    
    ///
    
    /// ```
    
    /// struct Dummy;
    
    ///
    
    /// impl LookupBot for Dummy {
    
    ///     type Request = ();
    
    ///     type Formatter = ();
    
    ///     type Response = String;
    
    ///
    
    ///     fn formatter(&self) -> Self::Formatter { Default::default() }
    
    ///     fn error_response() -> Self::Response { Default::default() }
    
    ///     fn empty_response() -> Self::Response { Self::Response::default() }
    
    ///     async fn answer(&self, _response: Self::Response) -> anyhow::Result<()> { Ok(()) }
    
    /// }
    
    ///
    
    /// // Use the default empty response
    
    /// let resp = Dummy::empty_response();
    
    /// assert_eq!(resp, String::default());
    
    /// ```
    fn empty_response() -> Self::Response {
        Self::Response::default()
    }

    async fn answer(&self, response: Self::Response) -> anyhow::Result<()>;
    /// Sends the bot's default error response.
    ///
    /// Attempts to send the response produced by `Self::error_response()`. Returns `Ok(())` if the response
    /// was sent successfully, or an `Err` if sending fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use async_trait::async_trait;
    /// # use crate::bot::lookup_bot::LookupBot;
    /// #[derive(Clone, Default)]
    /// struct MockFormatter;
    ///
    /// #[derive(Clone, Default)]
    /// struct MockResponse;
    ///
    /// struct MockBot;
    ///
    /// #[async_trait]
    /// impl LookupBot for MockBot {
    ///     type Request = String;
    ///     type Formatter = MockFormatter;
    ///     type Response = MockResponse;
    ///
    ///     async fn answer(&self, _response: Self::Response) -> anyhow::Result<()> {
    ///         // In a real bot this would send the response; here we succeed.
    ///         Ok(())
    ///     }
    /// }
    ///
    /// #[tokio::test]
    /// async fn sends_default_error_response() -> Result<()> {
    ///     let bot = MockBot;
    ///     bot.answer_generic_err().await?;
    ///     Ok(())
    /// }
    /// ```
    async fn answer_generic_err(&self) -> anyhow::Result<()> {
        self.answer(Self::error_response()).await?;
        Ok(())
    }

    /// Checks whether the given phrase is non-empty and emits an empty response when it is empty.
    ///
    /// If `phrase` is empty, sends `Self::empty_response()` via `answer` and returns `false`.
    /// Otherwise returns `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assume `bot` implements `LookupBot`.
    /// // let keep = bot.drop_empty("hello".to_string()).await;
    /// // assert!(keep);
    /// ```
    async fn drop_empty(&self, phrase: String) -> bool {
        match phrase.as_str() {
            "" => {
                let empty = Self::empty_response();
                let _ = self.answer(empty).await;
                false
            }
            _ => true,
        }
    }

    /// Convert a lookup `Result` into an `Option`, sending an error response when the result is an error.
    ///
    /// If `response` is `Ok(entity)`, returns `Some(entity)`. If `response` is `Err(_)`, attempts to send
    /// the trait's `error_response` via `answer`; if sending that response fails the failure is logged
    /// and `answer_generic_err` is attempted. In the error case, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn run_example<B: LookupBot>() {
    /// let bot: B = /* implementor */ todo!();
    /// let maybe = bot.ensure_request_success(Ok(42)).await;
    /// assert_eq!(maybe, Some(42));
    /// # }
    /// ```
    async fn ensure_request_success<Entity>(
        &self,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>
    where
        Entity: Send,
    {
        match response {
            Ok(values) => Some(values),
            Err(_) => {
                let resp = &self.answer(Self::error_response()).await;
                if let Err(e) = resp {
                    log::error!("Couldn't send error-response: {:?}", e);
                    let _ = &self.answer_generic_err().await;
                }
                None
            }
        }
    }
    /// Attempts to extract a successfully built response, sending a generic error reply on failure.
    ///
    /// If `response` is `Ok`, returns `Some` of the contained response. If `response` is `Err`,
    /// logs the error, attempts to send a generic error response via `answer_generic_err`, logs
    /// any failure to send that generic response, and returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given `bot` implements `LookupBot`, this demonstrates the expected usage.
    /// // let maybe_response = bot.retrieve_or_generic_err(build_result).await;
    /// // if let Some(response) = maybe_response {
    /// //     // use the successful response
    /// // }
    /// ```
    async fn retrieve_or_generic_err(
        &self,
        response: Result<Self::Response, LookupError>,
    ) -> Option<Self::Response> {
        match response {
            Ok(values) => Some(values),
            Err(err) => {
                log::error!("Failed to build response: {:?}", err);
                let result = self.answer_generic_err().await;
                if let Err(e) = result {
                    log::error!("Failed to respond generic err: {:?}", e);
                }
                None
            }
        }
    }

    /// Attempts to send the provided response using `answer`. If sending fails, logs the error and then tries to send the bot's generic error response.
    ///
    /// # Returns
    ///
    /// `Ok(())` after attempting to send the response; any send failures are handled internally and not propagated.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given an implementor `bot` of `LookupBot`:
    /// // bot.respond(response).await.unwrap();
    /// ```
    async fn respond(&self, response: Self::Response) -> anyhow::Result<()> {
        let res = self.answer(response).await;
        if let Err(e) = res {
            log::error!("Couldn't send response: {:?}", e);
            let _ = self.answer_generic_err().await;
        }
        Ok(())
    }
}