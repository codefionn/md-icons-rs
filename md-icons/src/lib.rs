//! Crate for using material design (SVG-) icons from
//! https://github.com/marella/material-design-icons as String constants or as maud
//! components.
//!
//! The icons have four different style: filled, outlined, sharp or two_tone. These are directly
//! represented as modules in this crate. You can get an overview of the icons at
//! [Google fonts](https://fonts.google.com/icons).
//!
//! ## Example
//!
//! Get the SVG as a string:
//!
//! ```rust
//! let hamburger_menu = md_icons::outlined::ICON_MENU;
//! ```
//!
//! ... or get it as a ``maud::Markup`` with the ``maud`` feature enabled:
//!
//! ```rust
//! let hamburger_menu = md_icons::outlined::maud_icon_menu();
//! ```

#[cfg(feature = "maud")]
extern crate maud;

extern crate md_icons_helper;

pub mod filled;
/// Outlined icons. See https://fonts.google.com/icons?icon.style=Outlined
pub mod outlined;
/// Sharp icons. See https://fonts.google.com/icons?icon.style=Sharp
pub mod sharp;
pub mod two_tone;

#[cfg(test)]
mod tests;
