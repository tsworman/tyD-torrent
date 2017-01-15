# tyD-torrent
A torrent directory cleaner implemented in Rust.

tyD-torrent cleans up a download directory from a previously downloaded Torrent. 
Takes input of the new version of the Torrent and the old download directory.
Passing -d or --delete will delete the files identified. The normal behavior is to list the files that should be deleted.

This was designed for use with Mame sets to catch up when multiple versions behind.
