#![no_std]

//! # Icons for Embedded and Resource Constrained Systems
//! The embedded-icon crate allows you to use over 7300 icons on all platforms and displays
//! that support [embedded_graphics].
//!
//! ## Usage
//!
//! ```rust
//! # use embedded_graphics::image::Image;
//! # use embedded_graphics::pixelcolor::{BinaryColor};
//! # use embedded_graphics::prelude::*;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut  display = MockDisplay::new();
//! // Import icons and traits
//! use embedded_icon::prelude::*;
//!
//! // Create an icon
//! let icon = icons::size24px::Download::new(BinaryColor::On);
//!
//! // Wrap it in an embedded_graphics image
//! let image = Image::new(&icon, Point::zero());
//!
//! // Draw it to a display
//! image.draw(&mut display).unwrap();
//! ```
//!
//! ## Storage Size
//! Using the crate will - without using any icons - not increase your binary size at all.
//! All icons that you use are automatically included in the binary. If you use an icon
//! multiple times, it'll only increase your binary size *once per resolution*.
//!
//! ## Resolutions
//! You can choose which resolutions to enable via features. Available resolutions are:
//!
//! | Resolution |   Module    | Bytes per Icon |
//! |------------|-------------|----------------|
//! | 12px       | [size12px]  | 18             |
//! | 18px       | [size18px]  | 41             |
//! | 24px       | [size24px]  | 72             |
//! | 32px       | [size32px]  | 128            |
//! | 48px       | [size48px]  | 288            |
//! | 96px       | [size96px]  | 1152 (== 1.2kb)|
//! | 144px      | [size144px] | 2592 (== 2.6kb)|
//!
//! ## Preview or find an icon
//! To see a preview of the included icons, please check out the
//! [Pictogrammers Website](https://pictogrammers.com/library/mdi/).//!
//!
//! ## Contributing
//!
//! If you found a bug, or think that a feature is missing, please open an issue on [GitHub](https://github.com/patrickelectric/embedded-icon).
//! Of course, Pull Requests are also very much appreciated.
//!
//!
//!

pub mod prelude {
    pub use crate::icon::*;
    pub use crate::icons;
}

mod icon;
pub use icon::*;

pub mod icons;
pub use icons::*;
