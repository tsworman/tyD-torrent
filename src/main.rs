extern crate bip_metainfo;
extern crate walkdir;
#[macro_use]
extern crate clap;

use std::fs::{self,File};
use std::path::PathBuf;
use std::io::Read;
use std::error::Error;
use std::collections::LinkedList;

use walkdir::WalkDir;
use bip_metainfo::MetainfoFile;

#[cfg(test)]
    mod tests {
    #[test]
	fn it_works() {
    }
}

//Main function - Read torrent file and compare to directory.
fn main() {
    let matches = clap_app!(tyD_torrent =>
			    (version: "1.0")
			    (author: "Tyler Worman <nova1313@novaslp.net")
			    (about: "Cleans a previously downloaded torrent directory by removing files not found in a newer verison of the same torrent. Designed for use with MAME sets.")
			    (@arg DELETE: -d --delete "Deletes the files found.")
			    (@arg INPUT: +required "The torrent to use.")
			    (@arg INPUTDIR: + required "The directory to scan for un-needed files.")
			    ).get_matches();

    let torrent_file_paths = read_torrent(matches.value_of("INPUT").unwrap()); //LinkedList::new
    
    //Read paths from the stated directory
    for paths in WalkDir::new(matches.value_of("INPUTDIR").unwrap()){
        let path_result = paths.unwrap();
        let path_string = path_result.path().display().to_string();
	let path_to_test = path_string.replace(matches.value_of("INPUTDIR").unwrap(), "");
        let path_buffer = PathBuf::from(path_to_test);
	
        let file_metadata = path_result.metadata().unwrap();
        let file_type = file_metadata.file_type();
	    
        if !file_type.is_dir() {
            if !torrent_file_paths.contains(&path_buffer) {
                println!("Delete: {}", path_buffer.display());

		if matches.is_present("DELETE") {
			match fs::remove_file(path_string) {
			    Err(why) => panic!("Can't delete: {}", why.description()),
				Ok(_) => {},
			}
	        }
            }
        }	
    }
}

fn read_torrent(torrent_file: &str) -> LinkedList<PathBuf> {
    let mut file = match File::open(torrent_file) {
        Err(why) => panic!("couldn't open {}: {}", torrent_file,
			   why.description()),
        Ok(file) => file,
    };
    
    let mut buffer = std::vec::Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", torrent_file,
    			   why.description()),
    	Ok(_) => {},
    }
    
    let mif = MetainfoFile::from_bytes(buffer).unwrap();
    let files = mif.info().files();
    let mut torrent_file_paths = LinkedList::new();
    
    for file in files {
	    let mut path_buffer  = PathBuf::from("");
	    for pp in file.paths() {
		    path_buffer.push(&pp);
	    }
	    torrent_file_paths.push_back(path_buffer);
    }

    torrent_file_paths

}
