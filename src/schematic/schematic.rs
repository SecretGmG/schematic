use std::{path::Path, fs::File};

use nbt::{Blob, to_gzip_writer};
use serde::Serialize;

use super::block_entity::BlockEntity;


///used for serialisation
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Schematic {
    pub(crate) width: i16,
    pub(crate) length: i16,
    pub(crate) height: i16,
    pub(crate) version: i32,
    pub(crate) data_version: i32,
    pub(crate) palette_max: i32,
    pub(crate) palette: Blob,
    pub(crate) block_entities: Vec<BlockEntity>,
    #[serde(serialize_with = "nbt::i8_array")]
    pub(crate) block_data: Vec<i8>,
}



impl Schematic {

    

    ///saves the schematic file to the given path
    pub fn save<P>(&self, path: P)
    where
        P: AsRef<Path>,
    {
        let mut file = File::create(path).unwrap();
        to_gzip_writer(&mut file, &self, None).unwrap();
    }
    
   
}