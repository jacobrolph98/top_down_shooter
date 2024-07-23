use bevy::{ecs::component::StorageType, prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;
use bevy_persistent::Persistent;

use crate::{
    game_loop::GameState, 
    interface::GenericButton, 
    persistence::Profiles
};

use super::{NewProfileRoute, MainMenuRoute};

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct ProfileListRoute;

pub struct ProfileListRoutePlugin;

impl Plugin for ProfileListRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, build_route.before(UiSystems::Compute).run_if(in_state(GameState::Profiles)));
    }
}

fn build_route(
    mut commands: Commands, 
    query: Query<Entity, Added<ProfileListRoute>>,
    profiles: Res<Persistent<Profiles>>,
    assets: Res<AssetServer>
) {
    for route_entity in &query {

        commands.entity(route_entity).insert(
            SpatialBundle::default()
        ).with_children(|route| {
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new2d("ProfileList")),
                MovableByCamera,
            )).with_children(|ui| {

                let root = UiLink::<MainUi>::path("Root");

                ui.spawn((
                    root.clone(),
                    UiLayout::window_full().pack::<Base>()
                ));

                let background = root.add("Background");
                ui.spawn((
                    background.clone(),
                    UiLayout::solid().size((2560.0, 1440.0)).scaling(Scaling::Fill).pack::<Base>(),
                    UiImage2dBundle::from(assets.load("ui/menu_background.png"))
                ));

                
                let panel = background.add("ProfilePanel");
                ui.spawn((
                    panel.clone(),
                    UiLayout::solid().size((9.0, 16.0)).pack::<Base>(),
                    UiImage2dBundle::from(assets.load("ui/profile_panel_background.png"))
                ));

                let list = panel.add("List");
                ui.spawn((
                    list.clone(),
                    UiLayout::window().pos(Rl((22.0, 33.0))).size(Rl((55.0, 34.0))).pack::<Base>()
                ));

                // No profiles so far
                for profile_data in profiles.profiles.iter() {
                    let profile_name = profile_data.profile_name.clone();
                    ui.spawn((
                        list.add(format!("PROFILE_{profile_name}")),
                        UiLayout::window().y(Rl(0.0)).size(Rl((100.0, 20.0))).pack::<Base>(),
                        GenericButton { text: profile_name },
                        OnUiClickDespawn::new(route_entity),
                        OnUiClickCommands::new(|commands| {
                            info!("Profile select clicked");
                            commands.spawn(MainMenuRoute);
                            // INIT RESOURCE FOR SELECTED PROFILE
                        })
                    ));
                }
                ui.spawn((
                    list.add(format!("NewProfile")),
                    UiLayout::window().y(Rl(0.0)).size(Rl((100.0, 20.0))).pack::<Base>(),
                    GenericButton { text: "New Profile".to_string() },
                    OnUiClickDespawn::new(route_entity),
                    OnUiClickCommands::new(|commands| {
                        info!("New profile clicked");
                        commands.spawn(NewProfileRoute);
                    })
                ));

            });
        });
        
    }
}