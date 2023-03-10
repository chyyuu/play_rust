extern crate libudev;

fn main() {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = libudev::Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("tty").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        println!("found device: {:?}", device.syspath());
    }
}
