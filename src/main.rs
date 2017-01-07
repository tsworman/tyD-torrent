extern crate bip_metainfo;

use std::fs::{File};
use std::path::{Path};
use std::io::{self, Write, BufRead};
use std::env;

use bip_metainfo::{MetainfoBuilder, MetainfoFile};
use bip_metainfo::error::{ParseResult};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       
    }
}

fn main() {
   if env::args().len() < 3 {
      help();
   }

   let mut args: Vec<_> = env::args().collect();
   println!("{}", args[1]);
}

fn help() {
  //Consider using docopt crate to handle all this!
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");
  println!("tyD-torrent v{}", VERSION);
  println!("Usage: tyD-torrent [somefile.torrent] [dir to scan]");
  println!("");
  println!("tyD-torrent cleans a directory by moving all files not listed in the torrent to a directory labled \"extraFiles\"");
  println!("This was developed to facilitate updating directories whose file listing had changed since the last update of that torrent");
} 