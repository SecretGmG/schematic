use itertools::Itertools;
use nbt::Blob;

use crate::schematic::schematic::Schematic;

use super::{blocks::air, block_builders::BlockBuilder, BB, Pos};

pub struct SchematicBuilder {
    blocks: Vec<Vec<Vec<Box<dyn BlockBuilder>>>>,
}

impl SchematicBuilder {
    const VERSION: i32 = 2;
    const DATA_VERSION: i32 = 2975;

    pub fn new(
        width: usize,
        length: usize,
        height: usize,
    ) -> SchematicBuilder {

        SchematicBuilder { blocks: Self::get_volume_of_air(width, length, height) }
    }
    
    fn get_volume_of_air(len1: usize, len2: usize, len3: usize) -> Vec<Vec<Vec<Box<dyn BlockBuilder>>>>{
        let mut v :Vec<Vec<Vec<Box<dyn BlockBuilder>>>> = vec![];
        for _ in 0..len1{
            v.push({
                let mut v :Vec<Vec<Box<dyn BlockBuilder>>> = vec![];
                for _ in 0..len2{
                    v.push({
                        let mut v :Vec<Box<dyn BlockBuilder>> = vec![];
                        for _ in 0..len3{
                            v.push(Box::new(air()));
                        }
                        v
                    });
                }
                v
            });
        }
        v
    }
    pub fn get_size(&self) -> (usize, usize, usize) {
        (
            self.blocks.len(),
            self.blocks[0].len(),
            self.blocks[0][0].len(),
        )
    }
    pub fn pos_iter(&self) -> impl Iterator<Item = (i32, i32, i32)>{
        let (x,y,z) = self.get_size();
        (0..x).cartesian_product(0..y).cartesian_product(0..z).map(|((x,y),z)| (x as i32,y as i32,z as i32))

    }
    pub fn get(&self, pos: Pos) -> &BB{
        &self.blocks[pos.0 as usize][pos.1 as usize][pos.2 as usize]
    }
    pub fn get_mut(&mut self, pos: Pos) -> &mut BB{
        &mut self.blocks[pos.0 as usize][pos.1 as usize][pos.2 as usize]
    }
    pub fn set(&mut self, pos :Pos, bb: BB){
        self.blocks[pos.0 as usize][pos.1 as usize][pos.2 as usize] = bb;
    }
    pub fn build(&self) -> Schematic {
        let mut palette_vec = vec![];
        let (width, length, height) = self.get_size();
        let mut block_data = vec![0; width * length * height];
        let mut block_entities = vec![];

        for ((x, y), z) in (0..width)
            .cartesian_product(0..length)
            .cartesian_product(0..height)
        {
            let block = &self.blocks[x][y][z];
            let pos = (x as i32,y as i32,z as i32);
            block_data[x + y * width + z * width * length] =
                match palette_vec.iter().position(|x| *x == block.get_tag(pos, self)) {
                    Some(id) => id.into(),
                    None => {
                        palette_vec.push(block.get_tag(pos, self));
                        palette_vec.len() - 1
                    }
                } as i8;
            block_entities.push(
                block
                    .get_block_entity(pos, self),
            );
        }
        let mut palette = Blob::new();
        for i in 0..palette_vec.len() {
            palette.insert(palette_vec[i].clone(), i as i32).unwrap();
        }
        Schematic {
            width: width as i16,
            length: length as i16,
            height: height as i16,
            version: Self::VERSION,
            data_version: Self::DATA_VERSION,
            palette_max: palette_vec.len() as i32,
            palette,
            block_entities: block_entities.into_iter().flatten().collect_vec(),
            block_data,
        }
    }
}
