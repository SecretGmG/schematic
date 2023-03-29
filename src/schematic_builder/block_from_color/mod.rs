use itertools::Itertools;
use kd_tree::KdTree;
use serde::{self, Deserialize, Serialize};
use super::block_builders::SimpleBlock;
mod block_builder_json;

#[derive(Debug)]
pub struct BlockSelector {
    blocks: KdTree<BlockColor>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlockColor {
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl kd_tree::KdPoint for BlockColor {
    type Scalar = i64;

    type Dim = typenum::U2;

    fn at(&self, i: usize) -> Self::Scalar {
        match i {
            0 => self.r.into(),
            1 => self.g.into(),
            2 => self.b.into(),
            _ => unreachable!(),
        }
    }
}
impl BlockSelector {
    pub fn new() -> BlockSelector {
        block_builder_json::get_block_selector()
    }
    pub(self) fn from_vec(blocks: Vec<BlockColor>) -> BlockSelector {
        BlockSelector {
            blocks: KdTree::build(blocks),
        }
    }
    ///returns a filtered block_selector only containing blocks where predicate evaluated to true
    pub fn filtered(&self, predicate: impl Fn(&String) -> bool) -> Self {
        BlockSelector::from_vec(
            self.blocks
                .items()
                .iter()
                .filter(|b| predicate(&b.name))
                .cloned()
                .collect_vec(),
        )
    }
    #[allow(dead_code)]
    ///Gets all the blocks this block selector has to offer
    pub fn blocks(&self) -> Vec<SimpleBlock> {
        return self
            .blocks
            .iter()
            .map(|b| SimpleBlock::new(format!("minecraft:{}", b.name)))
            .collect_vec();
    }
    ///Gets the block with the closest rgb value to the given one
    pub fn get_closest(&self, (r, g, b): (u8, u8, u8)) -> Option<SimpleBlock> {
        let item = self
            .blocks
            .nearest(&BlockColor {
                name: "".to_string(),
                r,
                g,
                b,
            })?
            .item;
        Some(SimpleBlock::new(format! {"minecraft:{}", item.name}))
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::schematic_builder::{
        block_from_color::BlockSelector, schematic_builder::SchematicBuilder,
    };

    #[test]
    fn load_new_block_selector() -> Result<(), Box<dyn Error>> {
        let block_selector = BlockSelector::new();
        dbg!(block_selector);
        Ok(())
    }
    #[test]
    fn create_test_schem() {
        let blocks = BlockSelector::new().blocks();
        let mut sb = SchematicBuilder::new(blocks.len(), 1, 1);
        dbg!(blocks.len());
        for i in 0..blocks.len() {
            sb.set((i as i32, 0, 0), Box::new(blocks[i].clone()));
        }
        sb.build().save("output/block_selector_test.schem");
    }
}
