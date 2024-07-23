use bevy::prelude::*;

pub mod generic_button;
pub use generic_button::*;

pub mod text_entry;
pub use text_entry::*;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((GenericButtonPlugin, TextEntryPlugin));
    }
}