use embedded_graphics::image::Image;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_icon::{
    iconoir::size32px::{
        Check, CoffeeCup, GlassEmpty, Icon3DSelectFace, PeaceHand, UserLove, XrayView,
    },
    ionic::size32px::{
        Airplane, BatteryHalfOutline, CaretForwardCircleOutline, CellularOutline,
        ChatbubbleOutline, ExtensionPuzzleOutline, RocketOutline,
    },
    mdi::size32px::{Abacus, Alarm, Cloud, Coffee, EmoticonHappy, HandHeart, Memory},
    simple::size32px::{
        Creativecommons, Github, Gnome, Homeassistant, Kde, Opensourceinitiative, Wikipedia,
    },
    NewIcon,
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
    let mut display = SimulatorDisplay::<Color>::new(Size::new(330, 210));

    // MDI
    draw_icon(&mut display, &Abacus::new(Rgb888::CSS_GOLD), 1, 1, 20, 20);
    draw_icon(&mut display, &Memory::new(Rgb888::CSS_GOLD), 3, 1, 20, 20);
    draw_icon(
        &mut display,
        &HandHeart::new(Rgb888::CSS_GOLD),
        5,
        1,
        20,
        20,
    );
    draw_icon(&mut display, &Cloud::new(Rgb888::CSS_GOLD), 7, 1, 20, 20);
    draw_icon(
        &mut display,
        &EmoticonHappy::new(Rgb888::CSS_GOLD),
        9,
        1,
        20,
        20,
    );
    draw_icon(&mut display, &Alarm::new(Rgb888::CSS_GOLD), 11, 1, 20, 20);
    draw_icon(&mut display, &Coffee::new(Rgb888::CSS_GOLD), 13, 1, 20, 20);

    // Iconoir
    draw_icon(&mut display, &Check::new(Rgb888::CSS_GREEN), 1, 3, 20, 20);
    draw_icon(
        &mut display,
        &CoffeeCup::new(Rgb888::CSS_GREEN),
        3,
        3,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &GlassEmpty::new(Rgb888::CSS_GREEN),
        5,
        3,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &Icon3DSelectFace::new(Rgb888::CSS_GREEN),
        7,
        3,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &PeaceHand::new(Rgb888::CSS_GREEN),
        9,
        3,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &UserLove::new(Rgb888::CSS_GREEN),
        11,
        3,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &XrayView::new(Rgb888::CSS_GREEN),
        13,
        3,
        20,
        20,
    );

    // Ionic
    draw_icon(&mut display, &Airplane::new(Rgb888::CSS_BLUE), 1, 5, 20, 20);
    draw_icon(
        &mut display,
        &BatteryHalfOutline::new(Rgb888::CSS_BLUE),
        3,
        5,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &ChatbubbleOutline::new(Rgb888::CSS_BLUE),
        5,
        5,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &CellularOutline::new(Rgb888::CSS_BLUE),
        7,
        5,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &CaretForwardCircleOutline::new(Rgb888::CSS_BLUE),
        9,
        5,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &ExtensionPuzzleOutline::new(Rgb888::CSS_BLUE),
        11,
        5,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &RocketOutline::new(Rgb888::CSS_BLUE),
        13,
        5,
        20,
        20,
    );

    //Simple
    draw_icon(
        &mut display,
        &Creativecommons::new(Rgb888::CSS_PURPLE),
        1,
        7,
        20,
        20,
    );
    draw_icon(&mut display, &Github::new(Rgb888::CSS_PURPLE), 3, 7, 20, 20);
    draw_icon(&mut display, &Gnome::new(Rgb888::CSS_PURPLE), 5, 7, 20, 20);
    draw_icon(
        &mut display,
        &Homeassistant::new(Rgb888::CSS_PURPLE),
        7,
        7,
        20,
        20,
    );
    draw_icon(&mut display, &Kde::new(Rgb888::CSS_PURPLE), 9, 7, 20, 20);
    draw_icon(
        &mut display,
        &Opensourceinitiative::new(Rgb888::CSS_PURPLE),
        11,
        7,
        20,
        20,
    );
    draw_icon(
        &mut display,
        &Wikipedia::new(Rgb888::CSS_PURPLE),
        13,
        7,
        20,
        20,
    );

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Example", &output_settings).show_static(&display);
}
