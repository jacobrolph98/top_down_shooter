pub use bevy::prelude::*;
pub use bevy_lunex::prelude::*;

mod components;
mod routes;

pub use {components::*, routes::*};

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((UiPlugin, ComponentPlugin, RoutePlugin));
    }
}