mod led;
mod rcio;

pub use {crate::led::Color, ms5611::Ms5611Sample as BarometerSample, ms5611::Osr as BarometerOsr};

use {crate::led::Led, embedded_hal::digital::v2::OutputPin, mpu9250::Mpu9250, ms5611::Ms5611};

pub struct Navio2<R, G, B> {
    led: Led<R, G, B>,
    barometer: Ms5611,
    //mpu9250: Mpu9250<D, M>,
    //lsm9ds1: LSM9DS1
    //m8n: U-blox M8N
    //rcio: rcio
}

impl<R, G, B> Navio2<R, G, B>
where
    R: OutputPin,
    G: OutputPin,
    B: OutputPin,
{
    pub fn new(led_pins: (R, G, B)) -> Self {
        Self {
            led: Led::new(led_pins.0, led_pins.1, led_pins.2),
            barometer: Ms5611::new(1, None).unwrap(),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.led.set(color);
    }

    pub fn barometer_sample(&mut self, osr: BarometerOsr) -> BarometerSample {
        self.barometer.read_sample(osr).unwrap()
    }
}
