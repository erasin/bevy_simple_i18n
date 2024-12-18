mod i18n_font;
#[cfg(feature = "numbers")]
mod i18n_number;
mod i18n_text;
mod i18n_text_2d;
mod utils;

pub use i18n_font::*;
#[cfg(feature = "numbers")]
pub use i18n_number::*;
pub use i18n_text::*;
pub use i18n_text_2d::*;

pub trait I18nComponent {
    /// If set, returns the locale of the component, otherwise the global locale
    fn locale(&self) -> String;

    /// Internal method that wraps the `rust_i18n::t!` macro
    fn translate(&self) -> String;
}
