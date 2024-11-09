use bevy::{
    asset::Handle,
    ecs::{reflect::ReflectResource, system::Resource},
    reflect::Reflect,
    text::Font,
    utils::hashbrown::HashMap,
};

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct I18n {
    locales: Vec<String>,
    current: String,
}

impl I18n {
    pub fn set_locale(&mut self, locale: impl Into<String>) {
        self.current = locale.into();
        rust_i18n::set_locale(&self.current);
    }

    pub fn current(&self) -> &str {
        &self.current
    }

    pub fn locales(&self) -> &[String] {
        &self.locales
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self {
            current: rust_i18n::locale().to_string(),
            locales: rust_i18n::available_locales!()
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }
}

impl std::fmt::Display for I18n {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "i18n | current: {}, locales: {:?}",
            self.current,
            self.locales.join(",")
        )
    }
}

#[derive(Debug, Default, Reflect)]
pub(crate) struct FontFolder {
    pub(crate) fallback: Handle<Font>,
    pub(crate) fonts: HashMap<String, Handle<Font>>,
}

impl FontFolder {
    pub(crate) fn get(&self, locale: impl Into<String>) -> Handle<Font> {
        let locale: String = locale.into();
        let mut locale = locale.as_str();

        bevy::log::debug!("Evaluating font for {} locale", locale);
        while !locale.is_empty() {
            if let Some(font) = self.fonts.get(locale) {
                bevy::log::debug!("Font for {} locale found", locale);
                return font.clone();
            }
            if let Some(index) = locale.rfind('-') {
                bevy::log::debug!("Font for {} locale was not found", locale);
                locale = &locale[..index];
            } else {
                break;
            }
        }

        bevy::log::debug!("Returning the fallback font");
        self.fallback.clone()
    }
}

#[derive(Debug, Reflect, Default, Resource)]
#[reflect(Resource)]
pub(crate) struct FontManager {
    fonts: HashMap<String, FontFolder>,
}

impl FontManager {
    pub(crate) fn insert(&mut self, family: impl Into<String>, font_folder: FontFolder) {
        let family: String = family.into();
        bevy::log::debug!("Font family {} added", family);
        self.fonts.insert(family, font_folder);
    }

    pub(crate) fn get(&self, family: &str, locale: Option<String>) -> Handle<Font> {
        let locale = locale.unwrap_or(rust_i18n::locale().to_string());
        self.fonts
            .get(family)
            .expect(format!("Font family {} not found", family).as_str())
            .get(locale)
    }
}
