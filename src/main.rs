#![allow(unstable)]
#![allow(unused)]

extern crate simple;

use simple::Engine;

fn main() {
    let e = Engine::new("November", 1920, 1080);
    e.run();
    e.quit();
}
