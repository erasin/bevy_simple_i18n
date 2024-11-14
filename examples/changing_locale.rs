use bevy::prelude::*;

use bevy_simple_i18n::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(I18nPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

fn setup(mut commands: Commands, i18n_res: Res<I18n>) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::Wrap,
            row_gap: Val::Px(10.),
            column_gap: Val::Px(10.),
            ..Default::default()
        })
        .with_children(|parent| {
            // Basic usage of the i18n text component
            parent
                .spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((I18nText::new("hello"), I18nFont::new("NotoSans")));
                });

            // Basic usage of the i18n number component
            parent
                .spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((I18nNumber::new(24501.20), I18nFont::new("NotoSans")));
                });

            // Example that shows variable interpolation
            parent
                .spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        I18nText::new("messages.hello").with_arg("name", "Bevy User"),
                        I18nFont::new("NotoSans"),
                    ));
                });

            // Example that shows variable interpolation with a number
            parent
                .spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        I18nText::new("messages.cats").with_num_arg("count", 2000.30),
                        I18nFont::new("NotoSans"),
                    ));
                });

            // Spawns a node with a text that will always be in Japanese
            parent
                .spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((Text::new("Always Japanese: "),));
                    parent.spawn((
                        I18nText::new("hello").with_locale("ja"),
                        I18nFont::new("NotoSans"),
                    ));
                });

            parent
                .spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(10.),
                    column_gap: Val::Px(10.),
                    ..default()
                })
                .with_children(|parent| {
                    for locale in i18n_res.locales() {
                        parent
                            .spawn((
                                Button,
                                Node {
                                    min_width: Val::Px(200.0),
                                    padding: UiRect::all(Val::Px(10.0)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor(Color::BLACK),
                                BorderRadius::MAX,
                                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            ))
                            .with_child((
                                Text::new(locale),
                                TextFont {
                                    font_size: 50.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                    }
                });
        });
}

fn button_system(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
    mut i18n_res: ResMut<I18n>,
) {
    for (interaction, children) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                let text = text_query.get(children[0]).unwrap().clone().0;
                i18n_res.set_locale(text);
            }
            _ => {}
        }
    }
}
