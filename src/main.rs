use serialport::{self, SerialPortInfo};
use slint::SharedString;
extern crate slint;

mod generate {
    slint::include_modules!();
}

pub use generate::*;

fn main() {
    let ui = generate::AppWindow::new();

    let serialports: Vec<SerialPortInfo> = serialport::available_ports().unwrap();

    let serialports: Vec<SharedString> = serialports
        .iter()
        .map(|s| {
            let s: Vec<&str> = s.port_name.split('/').collect();
            s[2].into()
        })
        .collect();

    ui.set_ports(slint::ModelRc::new(slint::VecModel::from(serialports)));

    ui.run();
}
