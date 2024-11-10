mod components;
mod plugin;
mod resources;

rust_i18n::i18n!("assets/locales");

pub mod prelude {
    pub use crate::components::*;
    pub use crate::plugin::I18nPlugin;
    pub use crate::resources::*;
}
