extern crate bip_metainfo;

use std::fs::File;
use std::path::{Path};
use std::io::{self, Write, Read, BufRead, prelude};
use std::env;
use std::error::Error;


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
	    //exit();
    }

    let mut args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();
    
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
			   why.description()),
        Ok(file) => file,
    };
    
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut buffer = std::vec::Vec::new();
    file.read_to_end(&mut buffer);
    //match file.read_to_string(&mut s) {
    //    Err(why) => panic!("couldn't read {}: {}", display,
    //			   why.description()),
    //	    Ok(_) => print!("Good!"),
    //	    //{} contains:\n{}", display, s),
    //	    }
    
    let mut mif = MetainfoFile::from_bytes(buffer).unwrap();
    // `file` goes out of scope, and the "hello.txt" file gets closed
    let mut files = mif.info().files();
    for file in files {
	    let mut path = String::from("");
	    for pp in file.paths() {
		    path.push('/');
		    path.push_str(pp);
		}
		println!("{}", path);
	}
    println!("{}", mif.info().is_private());
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


