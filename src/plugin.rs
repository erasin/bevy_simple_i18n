use std::path::Path;

use bevy::{
    app::{Plugin, PreStartup, Update},
    asset::Handle,
    ecs::{
        query::With,
        schedule::{common_conditions::resource_changed, IntoSystemConfigs},
        system::Query,
    },
    text::{Font, TextFont},
    ui::widget::Text,
};

use crate::{
    components::I18nText,
    prelude::I18nFont,
    resources::{FontFolder, FontManager, I18n},
};

include!(concat!(env!("OUT_DIR"), "/bevy_i18n.rs"));

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<I18n>()
            .init_resource::<FontManager>()
            .add_systems(PreStartup, load_dynamic_fonts)
            .add_systems(Update, update_translations.run_if(resource_changed::<I18n>));
    }
}

fn load_dynamic_fonts(
    mut font_manager: bevy::ecs::system::ResMut<FontManager>,
    asset_server: bevy::ecs::system::Res<bevy::asset::AssetServer>,
) {
    for dyn_font in FONT_FAMILIES.iter() {
        let mut font_folder = FontFolder::default();
        font_folder.fallback = asset_server.load(Path::new(dyn_font.path).join("fallback.ttf"));
        for font in dyn_font.locales.iter() {
            let locale = font.split('.').next().expect("Locale is required");
            let path = Path::new(dyn_font.path).join(font);
            let handler: Handle<Font> = asset_server.load(path);
            font_folder.fonts.insert(locale.to_string(), handler);
        }
        font_manager.insert(dyn_font.family.to_string(), font_folder);
    }
}

fn update_translations(
    font_manager: bevy::ecs::system::Res<FontManager>,
    mut text_query: Query<(&mut Text, &mut TextFont, Option<&I18nFont>, &I18nText), With<I18nText>>,
) {
    for (mut text, mut text_font, dyn_font, key) in text_query.iter_mut() {
        text.0 = key.translate();
        // bevy::log::info!("Text {} | {} | Font {:?}", **text, key, text_font.font);
        if let Some(dyn_font) = dyn_font {
            // bevy::log::info!("Updating dynamic font: {} for {}", dyn_font.0, **text);
            text_font.font = font_manager.get(&dyn_font.0, key.locale.clone());
        } else {
            // bevy::log::info!("No dynamic font found for: {}", **text);
        }
    }
}
