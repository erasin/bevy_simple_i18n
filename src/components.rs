use bevy::{
    ecs::{
        component::{Component, ComponentHooks, StorageType},
        reflect::ReflectComponent,
    },
    log::debug,
    reflect::Reflect,
    text::TextFont,
    ui::widget::Text,
};
use rust_i18n::t;

use crate::resources::*;

/// Component for spawning translatable text entities that are managed by `bevy_i18n`
///
/// It automatically inserts (or replaces) a Bevy `Text` component with the translated text using the provided key
///
/// Updates automatically whenever the locale is changed using the [I18n] resource
///
/// # Example
///
/// ```json
/// // en.json
/// {
///     "hello": "Hello, World!",
///     "greet": "Hello, %{name}!"
/// }
/// ```
///
/// ```
/// // Basic usage
/// world.spawn(I18nText::new("hello"));
///
/// // With interpolation arguments
/// world.spawn(I18nText::new("greet").with_arg("name", "Bevy User"));
///
/// // With forced locale
/// // overrides the global
/// // does not update when the locale is changed
/// world.spawn(I18nText::new("hello").with_locale("ja"));
/// ```
#[derive(Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct I18nText {
    pub(crate) key: String,
    pub(crate) args: Vec<(String, String)>,
    pub(crate) locale: Option<String>,
}

impl I18nText {
    /// Creates a new `I18nText` component with the provided translation key
    pub fn new(str: impl Into<String>) -> Self {
        Self {
            key: str.into(),
            args: vec![],
            locale: None,
        }
    }

    /// Add an interpolation argument to the translation key
    ///
    /// This method can be called as many times as needed
    pub fn with_arg(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.push((key.into(), value.into()));
        self
    }

    /// Set the locale for this specific translation
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// Internal method that wraps the `rust_i18n::t!` macro
    pub(crate) fn translate(&self) -> String {
        let (patterns, values): (Vec<&str>, Vec<String>) = self
            .args
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .unzip();
        let (patterns, values) = (patterns.as_slice(), values.as_slice());
        let translated = if let Some(locale) = self.locale.as_ref() {
            t!(&self.key, locale = locale)
        } else {
            t!(&self.key)
        };
        let val = rust_i18n::replace_patterns(&translated, patterns, values);
        val
    }
}

impl Component for I18nText {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, entity, _| {
            let val = world.get::<Self>(entity).unwrap().clone();
            debug!("Adding i18n text: {}", val.key);
            if let Some(mut text) = world.get_mut::<Text>(entity) {
                **text = val.translate();
            } else {
                world
                    .commands()
                    .entity(entity)
                    .insert(Text::new(val.translate()));
            }
        });
    }
}

impl std::fmt::Display for I18nText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "I18nText | key: {}, args: {}",
            self.key,
            self.args
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// Component for spawning dynamic font entities that are managed by `bevy_i18n`
///
/// The font for the text entity will be automatically updated based on the locale set by the [I18n] resource
///
/// # Example
///
/// ```
/// world.spawn((I18nText::new("hello"), I18nFont::new("NotoSans")));
/// ```
#[derive(Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct I18nFont(pub(crate) String);

impl I18nFont {
    /// Creates a new `I18nFont` component from the provided font family
    pub fn new(family: impl Into<String>) -> Self {
        Self(family.into())
    }
}

impl Component for I18nFont {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, entity, _| {
            let font_manager = world
                .get_resource::<FontManager>()
                .expect("Font manager has not been initialized");

            let locale = if let Some(i18n_text) = world.get::<I18nText>(entity) {
                i18n_text.locale.clone()
            } else {
                None
            };

            let val = world.get::<Self>(entity).unwrap().clone();
            let font_handler = font_manager.get(&val.0, locale);

            debug!("Adding dynamic font: {}", val.0);
            if let Some(mut font) = world.get_mut::<TextFont>(entity) {
                font.font = font_handler;
            } else {
                world.commands().entity(entity).insert(TextFont {
                    font: font_handler,
                    ..Default::default()
                });
            }
        });
    }
}
