mod components;
mod plugin;
mod resources;

include!(concat!(env!("OUT_DIR"), "/bevy_simple_i18n.rs"));

pub mod prelude {
    pub use crate::components::*;
    pub use crate::plugin::*;
    pub use crate::resources::*;
}
