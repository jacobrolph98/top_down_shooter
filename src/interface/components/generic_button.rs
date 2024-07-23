use bevy::{prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;

#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct GenericButton {
    pub text: String,
}

#[derive(Component, Debug, Default, Clone, PartialEq)]
struct GenericButtonUI;

pub struct GenericButtonPlugin;

impl Plugin for GenericButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiGenericPlugin::<GenericButtonUI>::new())
            .add_systems(Update, build_component.before(UiSystems::Compute));
    }
}

fn build_component(
    mut commands: Commands, 
    assets: Res<AssetServer>, 
    query: Query<(Entity, &GenericButton), Added<GenericButton>>
) {
    for (entity, src_button) in &query {
        commands.entity(entity).insert(
            UiTreeBundle::<GenericButtonUI>::from(UiTree::new2d("GenericButton"))
        ).with_children(|ui| {

            let background_img = ui.spawn((
                UiLink::<GenericButtonUI>::path("Control/Image"),
                UiLayout::window_full().pack::<Base>(),
                UiImage2dBundle::from(assets.load("ui/generic_button.png")),
                ImageScaleMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
                Pickable::IGNORE,
                UiAnimator::<Hover>::new().receiver(true),
                UiColor::<Base>::new(Color::srgb(0.5, 0.5, 0.5))
            )).id();

            let button_text = ui.spawn((
                UiLink::<GenericButtonUI>::path("Control/Image/Text"),
                UiLayout::window().pos(Rl((50., 50.))).anchor(Anchor::Center).pack::<Base>(),

                UiText2dBundle {
                    text: Text::from_section(&src_button.text,
                        TextStyle {
                            font_size: 60.0,
                            ..default()
                        }),
                    ..default()
                },
                Pickable::IGNORE,
                UiAnimator::<Hover>::new().receiver(true),
                UiColor::<Base>::new(Color::srgb(0.5, 0.5, 0.5))
            )).id();

            ui.spawn((
                UiLink::<GenericButtonUI>::path("Control"),
                UiLayout::window_full().pack::<Base>(),
                UiZoneBundle::default(),
                UiAnimator::<Hover>::new().forward_speed(5.0).backward_speed(1.0),
                UiAnimatorPipe::<Hover>::new(vec![button_text, background_img]),
                OnHoverSetCursor::new(CursorIcon::Pointer),
                UiClickEmitter::new(entity),
            ));
        });
    }
}