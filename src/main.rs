use clap::Parser;
use alsa::card::{Card, Iter};
use alsa::ctl::Ctl;
use alsa::rawmidi;
use libc::c_int;

/// ALSA MIDI utility
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List devices
    #[arg(short, long)]
    list_devices: bool,
}

fn main() {
    let args = Args::parse();

    if args.list_devices {
        device_list();
    }
}

fn device_list() {
    let mut cards = Iter::new();
    println!("Dir Device   Name");
    while let Some(result) = cards.next() {
        match result {
            Ok(card) => {
                list_card_devices(card.get_index());
            },
            Err(e) => {
                eprintln!("cannot determine card number: {}", e);
                return;
            }
        }
    }
}

fn list_card_devices(card: c_int) {
    let name = format!("hw:{}", card);
    let ctl = Ctl::new(name.as_str(), false);
    match ctl {
        Ok(c) => {
            let mut devices = rawmidi::Iter::new(&c);
            while let Some(result) = devices.next() {
                match result {
                    Ok(info) => {
                        list_device(&c, card, &info);
                    },
                    Err(_) => {
                        eprintln!("Couldn't get device");
                        return
                    }
                };
            };
        },
        Err(_) => {
            eprintln!("cannot open control for card {}: TODO: error", card);
            return;
        }
    }

    // TODO: Check that the wrapper closes the ctl when it is dropped
}

// This function is WIP. Can't make sense of the subdevice in / out counts
// or setting the stream on the info struct. Any hints are welcome.
fn list_device(ctl: &Ctl, card: c_int, info: &rawmidi::Info) {
    let sub_name = info.get_subdevice_name().unwrap();

    println!("Device {}, subdevice = {}, subdevice name = {}, stream = {:?}, ID = {}", 
        info.get_device(),
        info.get_subdevice(),
        sub_name,
        info.get_stream(), 
        info.get_id().unwrap());
}
