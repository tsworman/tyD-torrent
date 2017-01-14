extern crate bip_metainfo;
extern crate walkdir;
    
use std::fs::{File};
use std::path::{Path, PathBuf};
use std::io::{Read};
use std::env;
use std::error::Error;
use std::collections::LinkedList;

use walkdir::WalkDir;
use bip_metainfo::{MetainfoFile};

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

    let args: Vec<String> = env::args().collect();
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
    
    let mif = MetainfoFile::from_bytes(buffer).unwrap();
    let files = mif.info().files();
    let mut torrent_file_paths = LinkedList::new();
    
    for file in files {
	    let mut path_buffer  = PathBuf::from("");
	    for pp in file.paths() {
		    path_buffer.push(pp);
		}
	    torrent_file_paths.push_back(path_buffer);
	}
    
    //Read paths from the stated directory
    //was syspaths
    for paths in WalkDir::new(&args[2]){
        let path_result = paths.unwrap();
        let path_string = path_result.path().display().to_string();
        let path_to_test = path_string.replace(&args[2], "");
        let path_buffer = PathBuf::from(path_to_test);
    
        let file_metadata = path_result.metadata().unwrap();
        let file_type = file_metadata.file_type();
        println!("Trying to match: {}", path_buffer.display());
        if !file_type.is_dir() {
            if !torrent_file_paths.contains(&path_buffer) {
                println!("Delete: {}", path_buffer.display());
            }
        }	
    }
    //filePaths.contains(
}

fn read_torrent() {

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


