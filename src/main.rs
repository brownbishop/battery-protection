use clap::Parser;
use clap::ArgGroup;
use std::error::Error;
use std::fs;

/// Simple program to toggle battery protection mode on some Lenovo laptops.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("opts")
                .required(true)
                .args(&["enable", "disable", "status"]),
        ))]
struct Args {
    #[clap(short, long)]
    /// enable battery protection
    enable: bool,

    #[clap(short, long)]
    /// disable battery protection
    disable: bool,

    #[clap(short, long)]
    /// show protection status
    status: bool,
}

const LOCATION: &str = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";

fn status() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(LOCATION)?;

    match data.trim() {
        "1" => println!("battery protection enabled"),
        "0" => println!("battery protection disabled"),
        &_ => println!("unknown value"),
    };
    Ok(())
}

fn enable_protection() -> Result<(), Box<dyn Error>> {
    fs::write(LOCATION, "1")?;
    Ok(())
}

fn disable_protection() -> Result<(), Box<dyn Error>> {
    fs::write(LOCATION, "0")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();


    if args.enable {
        enable_protection()?;
    }
    if args.disable {
        disable_protection()?;
    }
    if args.status {
        status()?;
    }

    Ok(())
}
