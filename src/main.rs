extern crate argon2rs;
extern crate extra;
#[macro_use] extern crate failure;
extern crate syscall;

mod error;
mod group;
mod handle;
mod scheme;
mod user;

use std::io::{Read, Write};
use std::fs::File;

use extra::option::OptionalExt;
use syscall::data::Packet;
use syscall::scheme::SchemeMut;

use scheme::AuthScheme;

fn main() {
    if unsafe { syscall::clone(0).unwrap() } == 0 {
        let mut socket = File::create(":auth").unwrap_or_exit(1);
        let mut scheme = AuthScheme::new().unwrap_or_exit(1);
        
        //syscall::setrens(0, 0).unwrap_or_exit(1);
        
        loop {
            let mut packet = Packet::default();
            socket.read(&mut packet).unwrap_or_exit(1);
            
            scheme.handle(&mut packet);
            
            socket.write(&mut packet).unwrap_or_exit(1);
        }
    }
}
