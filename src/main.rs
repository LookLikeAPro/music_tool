use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

fn itunes_music_dir() -> PathBuf {
    let mut path = ::std::env::home_dir().unwrap();
    path.push("Music");
    path.push("iTunes");
    path.push("iTunes Media");
    path.push("Music");
    path
}

fn itunes_artists_folders(itunes_music_path: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut artists_folders = vec![];
    for entry in fs::read_dir(itunes_music_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            artists_folders.push(path);
        }
    }
    Ok(artists_folders)
}

fn check_albums(artists_folders: Vec<PathBuf>) -> Result<(), io::Error> {
    let mut hm: HashMap<String, ()> = HashMap::new();
    for folder in artists_folders {
        //        let artist = folder.clone().iter().last().unwrap();
        for entry in fs::read_dir(folder)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let album = path.iter().last().unwrap();
                let album_string = album.to_str().unwrap().to_string();
                match hm.entry(album_string.clone()) {
                    Entry::Occupied(o) => println!("{}", &album_string),
                    Entry::Vacant(v) => {
                        v.insert(());
                        ()
                    }
                };
            }
        }
    }
    Ok(())
}

fn main() {
    let dir = itunes_music_dir();
    let artists_folders = itunes_artists_folders(&dir).unwrap();
    check_albums(artists_folders);
}
