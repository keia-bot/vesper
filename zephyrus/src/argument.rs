use crate::hook::AutocompleteHook;
use crate::twilight_exports::*;

/// A command argument.
pub struct CommandArgument<D> {
    /// Argument name.
    pub name: &'static str,
    /// Description of the argument.
    pub description: &'static str,
    /// Whether the argument is required.
    pub required: bool,
    /// The type this argument has.
    pub kind: CommandOptionType,
    /// A function that allows to set specific options to the command, disabling arbitrary values.
    pub choices_fn: Box<dyn Fn() -> Option<Vec<CommandOptionChoice>> + Send + Sync>,
    /// A function used to autocomplete fields.
    pub autocomplete: Option<AutocompleteHook<D>>,
}

impl<D> CommandArgument<D> {
    fn choices(&self) -> Vec<CommandOptionChoice> {
        (self.choices_fn)().unwrap_or_default()
    }
    pub fn as_option(&self) -> CommandOption {
        match self.kind {
            CommandOptionType::String => CommandOption::String(ChoiceCommandOptionData {
                autocomplete: self.autocomplete.is_some(),
                choices: self.choices(),
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            CommandOptionType::Integer => CommandOption::Integer(NumberCommandOptionData {
                autocomplete: self.autocomplete.is_some(),
                choices: self.choices(),
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            CommandOptionType::Boolean => CommandOption::Boolean(BaseCommandOptionData {
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            CommandOptionType::User => CommandOption::User(BaseCommandOptionData {
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            CommandOptionType::Channel => CommandOption::Channel(ChannelCommandOptionData {
                channel_types: Vec::new(),
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            CommandOptionType::Role => CommandOption::Role(BaseCommandOptionData {
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            CommandOptionType::Mentionable => CommandOption::Mentionable(BaseCommandOptionData {
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            CommandOptionType::Number => CommandOption::Number(NumberCommandOptionData {
                autocomplete: self.autocomplete.is_some(),
                choices: self.choices(),
                description: self.description.to_string(),
                name: self.name.to_string(),
                required: self.required,
                ..Default::default()
            }),
            _ => unreachable!(),
        }
    }
}

impl<D>
    From<(
        &'static str,
        &'static str,
        bool,
        CommandOptionType,
        Box<dyn Fn() -> Option<Vec<CommandOptionChoice>> + Send + Sync>,
        Option<AutocompleteHook<D>>,
    )> for CommandArgument<D>
{
    fn from(
        (name, description, required, kind, fun, autocomplete): (
            &'static str,
            &'static str,
            bool,
            CommandOptionType,
            Box<dyn Fn() -> Option<Vec<CommandOptionChoice>> + Send + Sync>,
            Option<AutocompleteHook<D>>,
        ),
    ) -> Self {
        Self {
            name,
            description,
            required,
            kind,
            choices_fn: fun,
            autocomplete,
        }
    }
}
