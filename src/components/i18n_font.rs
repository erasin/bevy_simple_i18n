use bevy::{
    ecs::{
        component::{Component, ComponentHooks, StorageType},
        reflect::ReflectComponent,
    },
    log::debug,
    reflect::Reflect,
    text::TextFont,
};

use crate::{components::I18nNumber, resources::*};

use super::I18nText;

/// Component for spawning dynamic font entities that are managed by `bevy_simple_i18n`
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
            } else if let Some(i18n_number) = world.get::<I18nNumber>(entity) {
                i18n_number.locale.clone()
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
