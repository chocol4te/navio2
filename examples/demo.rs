use {
    navio2::{BarometerOsr, Color, Navio2},
    rppal::gpio::Gpio,
    std::{error::Error, thread, time::Duration},
};

fn main() -> Result<(), Box<dyn Error>> {
    let red = Gpio::new()?.get(4)?.into_output();
    let green = Gpio::new()?.get(27)?.into_output();
    let blue = Gpio::new()?.get(6)?.into_output();

    let mut navio2 = Navio2::new((red, green, blue));

    loop {
        println!(
            "color: off, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::Off);
        thread::sleep(Duration::from_millis(1000));

        println!(
            "color: red, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::Red);
        thread::sleep(Duration::from_millis(1000));

        println!(
            "color: green, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::Green);
        thread::sleep(Duration::from_millis(1000));

        println!(
            "color: blue, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::Blue);
        thread::sleep(Duration::from_millis(1000));

        println!(
            "color: cyan, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::Cyan);
        thread::sleep(Duration::from_millis(1000));

        println!(
            "color: magenta, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::Magenta);
        thread::sleep(Duration::from_millis(1000));

        println!(
            "color: yellow, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::Yellow);
        thread::sleep(Duration::from_millis(1000));

        println!(
            "color: white, baro: {:?}",
            navio2.barometer_sample(BarometerOsr::Opt1024)
        );
        navio2.set_color(Color::White);
        thread::sleep(Duration::from_millis(1000));
    }
}
