use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::interface::{GenericButton, TextEntry};

use super::{MainMenuRoute, ProfileListRoute};

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct NewProfileRoute;

pub struct NewProfileRoutePlugin;

impl Plugin for NewProfileRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

fn build_route(
    mut commands: Commands, 
    query: Query<Entity, Added<NewProfileRoute>>,
    assets: Res<AssetServer>
) {
    for route_entity in &query {
        commands.entity(route_entity).insert(
            SpatialBundle::default()
        ).with_children(|route| {
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new2d("NewProfile")),
                MovableByCamera,
            )).with_children(|ui| {


                let root = UiLink::<MainUi>::path("Root");
                ui.spawn((
                    root.clone(),
                    UiLayout::window_full().pack::<Base>()
                ));

                ui.spawn((
                    root.add("Background"),
                    UiLayout::solid().size((16.0, 9.0)).scaling(Scaling::Fill).pack::<Base>(),
                    UiImage2dBundle::from(assets.load("ui/menu_background.png"))
                ));

                
                let panel = root.add("NewProfilePanel");
                ui.spawn((
                    panel.clone(),
                    UiLayout::solid().size((881.0, 1600.0)).align_x(-0.74).pack::<Base>(),
                    UiImage2dBundle::from(assets.load("ui/profile_panel_background.png"))
                ));

                ui.spawn((
                    panel.add("ProfileNameEntry"),
                    TextEntry::new(|commands| {
                        commands.spawn(MainMenuRoute);
                    }, 
                    "".to_string(),
                    Some(route_entity)
                    )
                ));

                ui.spawn((
                    panel.add("Confirm"),
                    UiLayout::window().y(Rl(0.0)).size(Rl((100.0, 20.0))).pack::<Base>(),
                    GenericButton { text: "Confirm".to_string() },
                    OnUiClickDespawn::new(route_entity),
                    OnUiClickCommands::new(|commands| {
                        commands.spawn(MainMenuRoute);
                        // SOMETHING TO SELECT PROFILE 
                    })
                ));

                ui.spawn((
                    panel.add("Cancel"),
                    UiLayout::window().y(Rl(0.0)).size(Rl((100.0, 20.0))).pack::<Base>(),
                    GenericButton { text: "Cancel".to_string() },
                    OnUiClickDespawn::new(route_entity),
                    OnUiClickCommands::new(|commands| {
                        commands.spawn(ProfileListRoute);
                    })
                ));
            });

        });
        
    }
}