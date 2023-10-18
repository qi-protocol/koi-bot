use crate::tg_error;
use teloxide::{
    dispatching::dialogue::{Dialogue, InMemStorage},
    requests::Requester,
    types::Message,
    Bot,
};

type AddressPromptDialogue =
    Dialogue<AddressPromptDialogueState, InMemStorage<AddressPromptDialogueState>>;

#[derive(Clone, Debug, Default)]
pub(crate) enum AddressPromptDialogueState {
    #[default]
    StartAddressPrompt,
    ReceiveAddress,
    ReceiveTokenName,
}

pub(crate) async fn address_dialogue_handler(
    bot: &Bot,
    dialogue: AddressPromptDialogue,
    msg: &Message,
) -> Result<(), tg_error::TgError> {
    bot.send_message(
        msg.chat.id,
        "Enter the address of the token you want to trade",
    )
    .await?;
    dialogue
        .update(AddressPromptDialogueState::ReceiveTokenName)
        .await?;

    Ok(())
}
