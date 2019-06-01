use embedded_hal::digital::v2::OutputPin;

pub struct Led<R, G, B> {
    red: R,
    green: G,
    blue: B,
}

// Allow unused results until I decide on std/nostd
#[allow(unused_must_use)]
impl<R, G, B> Led<R, G, B>
where
    R: OutputPin,
    G: OutputPin,
    B: OutputPin,
{
    pub(crate) fn new(red: R, green: G, blue: B) -> Self {
        Self { red, green, blue }
    }

    pub(crate) fn set(&mut self, color: Color) {
        match color {
            Color::Off => {
                self.red.set_high();
                self.green.set_high();
                self.blue.set_high();
            }
            Color::Red => {
                self.red.set_low();
                self.green.set_high();
                self.blue.set_high();
            }
            Color::Green => {
                self.red.set_high();
                self.green.set_low();
                self.blue.set_high();
            }
            Color::Blue => {
                self.red.set_high();
                self.green.set_high();
                self.blue.set_low();
            }
            Color::Cyan => {
                self.red.set_high();
                self.green.set_low();
                self.blue.set_low();
            }
            Color::Magenta => {
                self.red.set_low();
                self.green.set_high();
                self.blue.set_low();
            }
            Color::Yellow => {
                self.red.set_low();
                self.green.set_low();
                self.blue.set_high();
            }
            Color::White => {
                self.red.set_low();
                self.green.set_low();
                self.blue.set_low();
            }
        }
    }
}

pub enum Color {
    Off,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,
}
