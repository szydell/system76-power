use dbus::{BusType, Connection, Message};
use dbus::arg::Append;
use std::io;

use {DBUS_NAME, DBUS_PATH, DBUS_IFACE, Power, err_str};
use backlight::{Backlight, BacklightExt};
use clap::ArgMatches;
use kbd_backlight::KeyboardBacklight;
use pstate::PState;

static TIMEOUT: i32 = 60 * 1000;

struct PowerClient {
    bus: Connection,
}

impl PowerClient {
    fn new() -> Result<PowerClient, String> {
        let bus = Connection::get_private(BusType::System).map_err(err_str)?;
        Ok(PowerClient { bus })
    }

    fn call_method<A: Append>(&mut self, method: &str, append: Option<A>) -> Result<Message, String> {
        let mut m = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, method)?;
        if let Some(arg) = append {
            m = m.append1(arg);
        }
        let r = self.bus.send_with_reply_and_block(m, TIMEOUT).map_err(err_str)?;
        Ok(r)
    }
}

impl Power for PowerClient {
    fn performance(&mut self) -> Result<(), String> {
        info!("Setting power profile to performance");
        self.call_method::<bool>("Performance", None)?;
        Ok(())
    }

    fn balanced(&mut self) -> Result<(), String> {
        info!("Setting power profile to balanced");
        self.call_method::<bool>("Balanced", None)?;
        Ok(())
    }

    fn battery(&mut self) -> Result<(), String> {
        info!("Setting power profile to battery");
        self.call_method::<bool>("Battery", None)?;
        Ok(())
    }

    fn get_graphics(&mut self) -> Result<String, String> {
        let r = self.call_method::<bool>("GetGraphics", None)?;
        r.get1().ok_or("return value not found".to_string())
    }

    fn set_graphics(&mut self, vendor: &str) -> Result<(), String> {
        info!("Setting graphics to {}", vendor);
        self.call_method::<&str>("SetGraphics", Some(vendor))?;
        Ok(())
    }

    fn get_graphics_power(&mut self) -> Result<bool, String> {
        let r = self.call_method::<bool>("GetGraphicsPower", None)?;
        r.get1().ok_or("return value not found".to_string())
    }

    fn set_graphics_power(&mut self, power: bool) -> Result<(), String> {
        info!("Turning discrete graphics {}", if power { "on" } else { "off "});
        self.call_method::<bool>("SetGraphicsPower", Some(power))?;
        Ok(())
    }

    fn auto_graphics_power(&mut self) -> Result<(), String> {
        info!("Setting discrete graphics to turn off when not in use");
        self.call_method::<bool>("AutoGraphicsPower", None)?;
        Ok(())
    }
}

fn profile() -> io::Result<()> {
    {
        let pstate = PState::new()?;
        let min = pstate.min_perf_pct()?;
        let max = pstate.max_perf_pct()?;
        let no_turbo = pstate.no_turbo()?;
        println!("CPU: {}% - {}%, {}", min, max, if no_turbo { "No Turbo" } else { "Turbo" });
    }

    for backlight in Backlight::all()? {
        let brightness = backlight.actual_brightness()?;
        let max_brightness = backlight.max_brightness()?;
        let ratio = (brightness as f64)/(max_brightness as f64);
        let percent = (ratio * 100.0) as u64;
        println!("Backlight {}: {}/{} = {}%", backlight.name(), brightness, max_brightness, percent);
    }

    for backlight in KeyboardBacklight::all()? {
        let brightness = backlight.brightness()?;
        let max_brightness = backlight.max_brightness()?;
        let ratio = (brightness as f64)/(max_brightness as f64);
        let percent = (ratio * 100.0) as u64;
        println!("Keyboard Backlight {}: {}/{} = {}%", backlight.name(), brightness, max_brightness, percent);
    }

    Ok(())
}

pub fn client(subcommand: &str, matches: &ArgMatches) -> Result<(), String> {
    let mut client = PowerClient::new()?;

    match subcommand {
        "profile" => match matches.value_of("profile") {
            Some("balanced") => client.balanced(),
            Some("battery") => client.battery(),
            Some("performance") => client.performance(),
            _ => profile().map_err(err_str)
        },
        "graphics" => match matches.subcommand() {
            ("intel", _) => client.set_graphics("intel"),
            ("nvidia", _) => client.set_graphics("nvidia"),
            ("power", Some(matches)) => match matches.value_of("state") {
                Some("auto") => client.auto_graphics_power(),
                Some("off") => client.set_graphics_power(false),
                Some("on") => client.set_graphics_power(true),
                _ => {
                    if client.get_graphics_power()? {
                        println!("on (discrete)");
                    } else {
                        println!("off (discrete)");
                    }
                    Ok(())
                }
            }
            _ => {
                println!("{}", client.get_graphics()?);
                Ok(())
            }
        }
        _ => Err(format!("unknown sub-command {}", subcommand))
    }
}