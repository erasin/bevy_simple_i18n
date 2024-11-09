mod components;
mod plugin;
mod resources;

#[macro_use]
extern crate rust_i18n;

i18n!("assets/locales");

pub mod prelude {
    pub use crate::components::*;
    pub use crate::plugin::I18nPlugin;
    pub use crate::resources::*;
}
