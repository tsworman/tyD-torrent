extern crate bip_metainfo;

use std::fs::{File};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write, Read, BufRead, prelude};
use std::env;
use std::error::Error;
use std::collections::LinkedList;

use bip_metainfo::{MetainfoBuilder, MetainfoFile};
use bip_metainfo::error::{ParseResult};

#[cfg(test)]
    mod tests {
    #[test]
	fn it_works() {
       
    }
}

//Main function - Read torrent file and compare to directory.
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
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display,
    			   why.description()),
    	    Ok(_) => {},
    }
    
    let mut mif = MetainfoFile::from_bytes(buffer).unwrap();
    let mut files = mif.info().files();
    let mut filePaths = LinkedList::new();
    
    for file in files {
	    let mut pathBuf  = PathBuf::from("");
	    for pp in file.paths() {
		     pathBuf.push(pp);
		}
	    println!("{}",pathBuf.display());
	    filePaths.push_back(pathBuf);
	}
    println!("{}", mif.info().is_private());

    //Read paths from the stated directory
    let sysPaths = fs::read_dir(&args[2]).unwrap();

    for paths in sysPaths {
	    let pathy = PathBuf::from(paths.unwrap().path().display().to_string());
	    let testPathy = pathy.display().to_string().replace(&args[2], "");
	    println!("Trying to match: {}", pathy.display());
	    if filePaths.contains(&pathy) {
		    println!("Matched!");
		} else {
		println!("Delete: {}", testPathy);
	    }
			
	}
    //filePaths.contains(
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

fn read_torrent() {

}

