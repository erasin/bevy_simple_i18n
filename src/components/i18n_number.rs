use bevy::{
    ecs::{
        component::{Component, ComponentHook, HookContext, Mutable, StorageType},
        reflect::ReflectComponent,
    },
    log::debug,
    reflect::Reflect,
    ui::widget::Text,
};
use fixed_decimal::FixedDecimal;

use super::{utils, I18nComponent};

/// Component for spawning translatable number entities that are managed by `bevy_simple_i18n`
///
/// It automatically inserts (or replaces) a Bevy `Text` component with the localized number
///
/// Updates automatically whenever the locale is changed using the [crate::resources::I18n] resource
///
/// # Example
///
/// ```
/// // Basic usage
/// world.spawn(I18nNumber::new(200.40));
///
/// // With forced locale
/// // overrides the global
/// // does not update when the locale is changed
/// world.spawn(I18nNumber::new(12051).with_locale("ja"));
/// ```
#[derive(Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct I18nNumber {
    #[reflect(ignore)]
    pub(crate) fixed_decimal: FixedDecimal,
    /// Locale for this specific translation, `None` to use the global locale
    pub(crate) locale: Option<String>,
}

impl I18nComponent for I18nNumber {
    fn locale(&self) -> String {
        self.locale
            .clone()
            .unwrap_or(rust_i18n::locale().to_string())
    }

    fn translate(&self) -> String {
        utils::get_formatter(&self.locale(), &self.fixed_decimal)
            .format_to_string(&self.fixed_decimal)
    }
}

impl I18nNumber {
    /// Creates a new `I18nNumber` component with the provided number value
    pub fn new(number: impl Into<f64>) -> Self {
        Self {
            fixed_decimal: utils::f64_to_fd(number.into()),
            locale: None,
        }
    }

    /// Set the locale for this specific translation
    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
}

impl Component for I18nNumber {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, HookContext { entity, .. }| {
            let val = world.get::<Self>(entity).unwrap().clone();
            debug!("Adding i18n number: {}", val.fixed_decimal);
            if let Some(mut text) = world.get_mut::<Text>(entity) {
                **text = val.translate();
            } else {
                world
                    .commands()
                    .entity(entity)
                    .insert(Text::new(val.translate()));
            }
        })
    }
}
