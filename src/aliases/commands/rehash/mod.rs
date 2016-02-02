use std::path::PathBuf;
use std::fs;

use aliases::factories::{AliasFactory, ShimFileFactory};

pub struct Rehash {
    pub shim_directory: PathBuf,
    pub alias_directories: Vec<PathBuf>,
}

impl Rehash {

    pub fn new(shim_directory: PathBuf, alias_directories: Vec<PathBuf>) -> Rehash {
        Rehash { 
            shim_directory: shim_directory,
            alias_directories: alias_directories,
        }
    }

    pub fn execute(&self) {
        self.create_shims_directory();
        for dir in &self.alias_directories {
            match AliasFactory::create_from_file(dir.join(".aliases")) {
                Err(_) => {}, // TODO
                Ok(aliases) => {
                    for alias in aliases.raw_collection.iter() { // i know don't touch the raw collection have to fix this.
                        ShimFileFactory::create(&alias, &self.shim_directory);
                    }
                }
            }
        }
    }

    //--------  private ----------//

    fn create_shims_directory(&self) {
        fs::create_dir_all(&self.shim_directory);
    }
}
