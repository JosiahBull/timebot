use std::str::FromStr;

use serenity::{
    all::CommandInteraction,
    async_trait,
    builder::{CreateCommand, CreateCommandOption},
    prelude::Context,
};

use crate::state::AppState;

use super::{command::Command, util::CommandResponse};

pub struct TimeCommand;

impl<'a> TryFrom<&'a CommandInteraction> for TimeCommand {
    type Error = String;
    fn try_from(_: &'a CommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

#[async_trait]
impl<'a> Command<'a> for TimeCommand {
    fn name() -> &'static str {
        "time"
    }

    fn description() -> &'static str {
        "Creates a large message to hide previous messages in the chat"
    }

    fn get_application_command_options(i: CreateCommand) -> CreateCommand {
        i
            .add_option(CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "location",
                "The location to get the time for",
            )
            .required(true)
            .add_string_choice("London", "Europe/London")
            .add_string_choice("Auckland", "Pacific/Auckland")
        )
    }

    #[allow(clippy::invisible_characters)]
    async fn handle_application_command<'b>(
        self,
        interaction: &'b CommandInteraction,
        _: &'b AppState,
        _: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        // load the location from the command
        let location = &interaction.data.options.get(0).unwrap().value;
        let location = location.as_str().unwrap();

        // get the current time
        let now = chrono::Local::now().with_timezone(&chrono_tz::Tz::from_str(location).unwrap());

        // create the response, should be "The time in Auckland is 3:34pm on a Tuesday"
        let response_str = format!("The time in {} is{} on a {}", location, now.format("%l:%M%P"), now.format("%A"));

        Ok(CommandResponse::BasicSuccess(response_str))
    }
}
