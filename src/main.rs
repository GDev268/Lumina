extern crate ash;
extern crate winit;

mod game;
mod window;
mod device;

use crate::device::Device;


fn main() {
    println!("Hello World!");
    let device = Device::new();

}
