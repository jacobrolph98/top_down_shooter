use bevy::prelude::*;

pub mod profile_list;
pub use profile_list::*;

pub mod new_profile;
pub use new_profile::*;

pub mod main_menu;
pub use main_menu::*;

pub struct RoutePlugin;

impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ProfileListRoutePlugin, 
                NewProfileRoutePlugin,
                MainMenuRoutePlugin
            ));
    }
}