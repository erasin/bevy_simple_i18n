use bevy::prelude::*;

use bevy_i18n::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the base plugin
        .add_plugins(I18nPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                // i18n text component with key "hello"
                I18nText::new("hello"),
                // Dynamic font component with font family "NotoSans" that auto loads font files based on the set locale
                I18nFont::new("NotoSans"),
                // You can still insert a TextFont component though
                // Just keep in mind that the "font" field will be overridden by the I18nFont component
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}
