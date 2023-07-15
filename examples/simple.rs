use embedded_graphics::image::Image;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_icon::{
    iconoir::size32px::AddCircle, ionic::size32px::Airplane, mdi::size32px::Abacus,
    simple::size32px::Kde, NewIcon,
};
type Color = Rgb888;

fn draw_icon(
    display: &mut impl DrawTarget<Color = Color>,
    icon: &impl ImageDrawable<Color = Color>,
    xpos: u32,
    ypos: u32,
    xincr: u32,
    yincr: u32,
) {
    Image::new(
        icon,
        Point::new((10 + xincr * xpos) as i32, (10 + yincr * ypos) as i32),
    )
    .draw(display)
    .ok();
}

fn main() {
    let mut display = SimulatorDisplay::<Color>::new(Size::new(320, 240));

    draw_icon(&mut display, &Abacus::new(Rgb888::CSS_GOLD), 1, 1, 20, 20);
    draw_icon(
        &mut display,
        &AddCircle::new(Rgb888::CSS_GOLD),
        3,
        1,
        20,
        20,
    );
    draw_icon(&mut display, &Airplane::new(Rgb888::CSS_GOLD), 5, 1, 20, 20);
    draw_icon(&mut display, &Kde::new(Rgb888::CSS_GOLD), 7, 1, 20, 20);

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Examplea", &output_settings).show_static(&display);
}
