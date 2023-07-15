use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

#[macro_export]
macro_rules! include_icon {
    ($icon_name:ident, $group:ident, $filename:literal, $size:literal) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $icon_name;

        impl $crate::icon::sealed::IconInternal for $icon_name {
            const SIZE: u32 = $size;
            const DATA: &'static [u8] =
                include_bytes!(concat!(stringify!($size), "px/", $filename, ".bits"));
            const INSTANCE: Self = $icon_name;
        }

        static_assertions::const_assert!(
            <$icon_name as $crate::icon::sealed::IconInternal>::DATA.len()
                >= <$icon_name as $crate::icon::sealed::IconInternal>::BYTE_COUNT
        );
    };
}

/// Struct to store icon color and properties.
///
/// There are two ways to instantiate an icon:
/// ```rust
/// # use embedded_graphics::pixelcolor::BinaryColor;
/// # use embedded_icon::Icon;
/// # use embedded_icon::prelude::*;
/// // using constructors on icons (recommended)
/// let icon = icons::size24px::actions::Download::new(BinaryColor::On);
/// // using types
/// let icon: Icon<_, icons::size24px::actions::Download> = Icon::new(BinaryColor::On);
/// ```
/// Both result in the same icon (`Icon<COLOR, ICON>`). Use whichever you prefer.
///
///
///
/// ## Full Usage Example
///
/// ```rust
/// # use embedded_graphics::image::Image;
/// # use embedded_graphics::pixelcolor::{BinaryColor};
/// # use embedded_graphics::prelude::*;
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut  display = MockDisplay::new();
/// // Import icons and traits
/// use embedded_icon::prelude::*;
///
/// // Create an icon
/// let icon = icons::size24px::actions::Download::new(BinaryColor::On);
///
/// // Wrap it in an embedded_graphics image
/// let image = Image::new(&icon, Point::zero());
///
/// // Draw it to a display
/// image.draw(&mut display).unwrap();
/// ```
#[derive(Debug)]
pub struct Icon<C, Ico>
where
    C: PixelColor,
    Ico: sealed::IconInternal,
{
    color: C,
    _icon: Ico,
}

impl<C: PixelColor, Ico: sealed::IconInternal> Icon<C, Ico> {
    pub fn new(color: C) -> Self {
        Self {
            color,
            _icon: Ico::INSTANCE,
        }
    }

    pub fn set_color(&mut self, color: C) {
        self.color = color;
    }

    pub fn get_color(&self) -> C {
        self.color
    }
}

/// Marker Trait for all Icons
pub trait EmbeddedIcon: Sized + sealed::IconInternal {}

impl<T> EmbeddedIcon for T where T: sealed::IconInternal {}

pub trait NewIcon<C: embedded_graphics::prelude::PixelColor>: Sized
where
    Self: sealed::IconInternal,
{
    fn new(color: C) -> Icon<C, Self>;
}

impl<C: PixelColor, T> NewIcon<C> for T
where
    T: sealed::IconInternal,
{
    fn new(color: C) -> Icon<C, Self> {
        Icon {
            color,
            _icon: Self::INSTANCE,
        }
    }
}

pub(crate) mod sealed {
    pub trait IconInternal: Sized {
        const SIZE: u32;
        const BIT_COUNT: usize = { Self::SIZE as usize * Self::SIZE as usize };
        const BYTE_COUNT: usize =
            { Self::BIT_COUNT / 8 + if Self::BIT_COUNT % 8 > 0 { 1 } else { 0 } };
        const DATA: &'static [u8];
        const INSTANCE: Self;
    }
}

impl<C, T> ImageDrawable for Icon<C, T>
where
    T: sealed::IconInternal,
    C: PixelColor,
{
    type Color = C;
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let data = T::DATA;
        for y in 0..T::SIZE {
            for x in 0..T::SIZE {
                if get_bit_unchecked(data, (x + y * T::SIZE) as usize) {
                    Pixel(Point::new(x as i32, y as i32), self.get_color()).draw(target)?;
                }
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // from tinytga
        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}

impl<C, T> OriginDimensions for Icon<C, T>
where
    T: sealed::IconInternal,
    C: PixelColor,
{
    #[inline(always)]
    fn size(&self) -> Size {
        Size {
            width: T::SIZE,
            height: T::SIZE,
        }
    }
}

/// Retrieve the n-th bit from a slice of bytes
/// without performing in-bounds checking
fn get_bit_unchecked(target: &[u8], bit: usize) -> bool {
    let slice_index = bit / 8;
    let bit_index = bit % 8;
    (target[slice_index] & (1 << bit_index)) != 0
}
