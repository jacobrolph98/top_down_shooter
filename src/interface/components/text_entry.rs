use bevy::{input::{keyboard::{Key, KeyboardInput}, ButtonState}, prelude::*};
use bevy_lunex::prelude::*;

#[derive(Component, Clone, PartialEq, Eq)]
pub struct TextEntry {
    pub confirm_closure: fn(&mut Commands),
    pub text: String,
    pub route_despawner: Option<Entity>
}

impl TextEntry {
    pub fn new(confirm_closure: fn(&mut Commands), text: String, route_despawner: Option<Entity>) -> Self {
        TextEntry {
            confirm_closure,
            text,
            route_despawner
        }
    }
}

#[derive(Component)]
struct TextEntryUI;


pub struct TextEntryPlugin;

impl Plugin for TextEntryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiGenericPlugin::<TextEntryUI>::new())
            .add_systems(Update, build_route.before(UiSystems::Compute))
            .add_systems(Update, text_fill.run_if(text_entry_exists));
    }
}

fn text_entry_exists(
    text_entry_query: Query<Entity, With<TextEntry>>
) -> bool {
    !text_entry_query.is_empty()
}

fn build_route(
    mut commands: Commands, 
    assets: Res<AssetServer>, 
    query: Query<Entity, Added<TextEntry>>
) {
    for entity in &query {
        commands.entity(entity).insert(
            UiTreeBundle::<TextEntryUI>::from(UiTree::new2d("TextEntry"))
        ).with_children(|ui| {
            ui.spawn((
                UiLink::<TextEntryUI>::path("Text"),
                UiLayout::window_full().pack::<Base>(),
                UiText2dBundle {
                    text: Text::from_section("",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..default()
                    }),
                ..default()
                },
                PickableBundle::default()
            ));
        });
    }
}

fn text_fill(
    mut key_reader: EventReader<KeyboardInput>,
    mut input_field: Local<String>,
    mut text_entry_query: Query<(&mut Text, &TextEntry)>,
    mut commands: Commands
) {
    for kb_input in key_reader.read() {
        if kb_input.state == ButtonState::Released { continue; }
        let (mut field_text, entry_field) = text_entry_query.single_mut();
        match &kb_input.logical_key {

            Key::Enter => {
                if input_field.is_empty(){
                    info!("Empty profile name");
                    continue;
                }
                if let Some(route) = entry_field.route_despawner {
                    commands.entity(route).despawn_recursive();
                }
                (entry_field.confirm_closure)(&mut commands);
            }

            Key::Backspace => {
                input_field.pop();
            }

            Key::Character(input) => {
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                input_field.push_str(&input);
                field_text.sections[0].value.push_str(&input);
            }

            _ => {}
        }
        
    }
}