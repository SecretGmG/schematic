use self::block_builders::BlockBuilder;
pub mod schematic_builder;
pub mod blocks;
pub mod block_builders;
pub mod block_from_color;

pub type Pos = (i32, i32, i32);
pub type BB = Box<dyn BlockBuilder>;