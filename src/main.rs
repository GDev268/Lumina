extern crate ash;
extern crate winit;

mod game;
mod window;
mod device;

use crate::device::Device;


fn main() {
    let device = Device::none(false);

}
