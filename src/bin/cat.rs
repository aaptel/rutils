use std::env;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[macro_use]
extern crate log;
extern crate env_logger;

fn cat_read<T: Read>(h: &mut T) {
    // use unitialized stack buffer
    let mut buf: [u8; 1024] = unsafe { std::mem::uninitialized() };
    let mut i = 0;
    loop {
        match h.read(&mut buf) {
            Err(e) => {
                error!("err while reading: {}", e.description());
                return;
            }
            Ok(0) => {
                info!("read all bytes, done");
                return;
            }
            Ok(n) => {
                info!("#{} read {} bytes", i, n);
            }
        }
        i += 1;
    }
}

fn main() {
    env_logger::init().unwrap();

    if env::args().len() <= 1 {
        info!("reading stdin");
        cat_read(&mut std::io::stdin())
    } else {
        for arg in env::args().skip(1) {
            match File::open(&arg) {
                Err(e) => {
                    error!("cannot open file {}: {}", arg, e);
                    return;
                }
                Ok(mut h) => {
                    cat_read(&mut h);
                }
            }
        }
    }
}
