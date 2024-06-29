use std::env;

use completion::{CompletionMessage, CompletionRequest, CompletionResponse};
use reqwest::Client;

use crate::{
    consts::{
        CHAT_GPT_API_KEY, COMPLETION_PROMPT_TEMPLATE, GPT_MODEL, GPT_MSG_ROLE_USER, GPT_URL,
        HEADER_AUTHORIZATION, HEADER_BEARER, NO_CHAT_GPT_API_KEY, NO_GPT_MODEL,
    },
    Rating,
};

pub use error::*;

mod completion;
mod error;

pub struct CommitRater {
    gpt_api_key: String,
    client: Client,
}

impl Default for CommitRater {
    fn default() -> Self {
        let gpt_api_key = env::var(CHAT_GPT_API_KEY).expect(NO_CHAT_GPT_API_KEY);

        let client = Client::new();

        CommitRater {
            gpt_api_key,
            client,
        }
    }
}

impl CommitRater {
    pub async fn rate_commit(&self, commit_msg: String) -> Result<Rating> {
        let gpt_model = env::var(GPT_MODEL).expect(NO_GPT_MODEL);

        let msg_content = format!("{}\n{}", COMPLETION_PROMPT_TEMPLATE, commit_msg);
        let msg = CompletionMessage {
            role: GPT_MSG_ROLE_USER.to_string(),
            content: msg_content,
        };
        let completion_request = CompletionRequest {
            model: gpt_model,
            messages: vec![msg],
        };

        let resp = self
            .client
            .post(GPT_URL)
            .header(
                HEADER_AUTHORIZATION,
                format!("{}{}", HEADER_BEARER, self.gpt_api_key),
            )
            .json(&completion_request)
            .send()
            .await?
            .json::<CompletionResponse>()
            .await?;

        // Handle the response
        if let Some(choice) = resp.choices.first() {
            let rating: Rating = serde_json::from_str(&choice.message.content)?;

            Ok(rating)
        } else {
            Err(Error::NoGPTRating)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CommitRater;
    use dotenv::dotenv;

    fn setup() {
        dotenv().ok();
    }

    #[tokio::test]
    async fn test_rate_commit() {
        // -- Setup
        setup();

        // -- Create commit rater
        let commit_rater = CommitRater::default();

        // -- Create example commit msg
        let commit_msg = "[`f1b798d`](https://github.com/EmptyBagGames/RollTheBall/commit/f1b798d0e0337dd371899a7f0f8415dea1afcc75) added simple conveyor belt platform - mako-lcl";

        let _ = commit_rater.rate_commit(commit_msg.to_string()).await;
    }
}
