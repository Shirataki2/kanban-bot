use super::help::help;
use serde_json::Value;
use serenity::model::{channel::GuildChannel, id::UserId, interactions::ApplicationCommandInteractionData};

#[derive(Debug, Default)]
/// # Note
///
/// If `ephemeral` is set to true, Embeds will not be sent.
pub struct Response {
    pub message: String,
    pub embeds: Vec<Value>,
    pub ephemeral: bool,
}

pub enum Operation {
    I18n,
}

impl Operation {
    pub async fn apply(
        &self,
        ctx: &serenity::client::Context,
        data: &ApplicationCommandInteractionData,
        user_id: UserId,
        text_channel: &GuildChannel,
    ) -> anyhow::Result<Option<Response>> {
        let out = match self {
            Operation::I18n => help(ctx, data, user_id, text_channel).await?,
        };
        Ok(out)
    }
}
