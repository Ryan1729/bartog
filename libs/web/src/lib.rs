#![recursion_limit = "2048"] // Only needed for "stdweb_version"

#[cfg(feature = "stdweb_version")]
#[macro_use]
extern crate stdweb;

#[cfg(feature = "stdweb_version")]
mod stdweb_version;

#[cfg(feature = "stdweb_version")]
pub use stdweb_version::{get_state_params, run};

#[cfg(feature = "web_sys_version")]
mod web_sys_version;

#[cfg(feature = "web_sys_version")]
pub use web_sys_version::{get_state_params, run};

#[cfg(any(
    not(any(feature = "stdweb_version", feature = "web_sys_version")),
    all(feature = "stdweb_version", feature = "web_sys_version")
))]
compile_error!{
    "You must pick exactly one of either the \"stdweb_version\" or \"web_sys_version\" features"
}