use std::process;

use anyhow::Result;
use clap::arg_enum;
use clap::StructOpt;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum TemperatureUnit {
        Fahrenheit,
        Celsius,
        Kelvin,
        F,
        C,
        K
    }
}

/// Convert temperature units as if by magic.
#[derive(Debug, StructOpt)]
#[structopt(name = "Temperature Converter", about = "L33t h4x")]
struct Cli {
    /// The desired output units.
    #[structopt(
        possible_values = TemperatureUnit::variants(), case_insensitive = true,
        long = "to",
        short = 'w'
    )]
    to: TemperatureUnit,

    /// The input units.
    #[structopt(
        possible_values = TemperatureUnit::variants(), case_insensitive = true,
        long = "from",
        short = 'r',
        default_value = "Fahrenheit"
    )]
    from: TemperatureUnit,

    #[structopt(name = "VALUE")]
    value: f64,
}

fn long_form(u: &TemperatureUnit) -> TemperatureUnit {
    match u {
        TemperatureUnit::F => TemperatureUnit::Fahrenheit,
        TemperatureUnit::C => TemperatureUnit::Celsius,
        TemperatureUnit::K => TemperatureUnit::Kelvin,
        _ => u.clone(),
    }
}

fn finish(v: f64) {
    println!("{:.*}", 2, v);
    process::exit(0);
}

fn main() -> Result<()> {
    // Select different input units if the output is using the input's default.
    let args = Cli::from_args();
    let input = match &args.to {
        TemperatureUnit::Fahrenheit | TemperatureUnit::F => TemperatureUnit::Celsius,
        _ => args.from,
    };

    let from = long_form(&input);
    let to = long_form(&args.to);

    match (from, to) {
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => {
            finish((&args.value - 32.0) * 5.0 / 9.0);
        }
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Kelvin) => {
            finish((&args.value - 32.0) * 5.0 / 9.0 + 273.15);
        }
        (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => {
            finish((&args.value * 9.0 / 5.0) + 32.0);
        }
        (TemperatureUnit::Celsius, TemperatureUnit::Kelvin) => {
            finish(&args.value + 273.15);
        }
        (TemperatureUnit::Kelvin, TemperatureUnit::Fahrenheit) => {
            finish(((&args.value - 273.15) - 32.0) * 9.0 / 5.0);
        }
        (TemperatureUnit::Kelvin, TemperatureUnit::Celsius) => {
            finish(&args.value - 273.15);
        }
        _ => finish(args.value),
    };
    Ok(())
}
