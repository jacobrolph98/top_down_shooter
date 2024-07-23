use bevy::prelude::*;
use bevy_lunex::prelude::*;

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MainMenuRoute;

pub struct MainMenuRoutePlugin;

impl Plugin for MainMenuRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}

fn build_route(
    mut commands: Commands, 
    query: Query<Entity, Added<MainMenuRoute>>,

) {
    for route_entity in &query {
        commands.entity(route_entity).insert(
            SpatialBundle::default()
        ).with_children(|route| {
            route.spawn((
                UiTreeBundle::<MainUi>::from(UiTree::new2d("MainMenu")),
                MovableByCamera,
            )).with_children(|ui| {
                let root = UiLink::<MainUi>::path("Root");
                ui.spawn((
                    root.clone(),
                    UiLayout::window_full().pack::<Base>()
                ));
                // Button for missions
                // Button for shop
                // Button for settings
                // Button for deleting profile
                // Button for exiting game
            });

        });
        
    }
}