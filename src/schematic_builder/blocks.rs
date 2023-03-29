use super::block_builders::{SimpleBlock, BlockWithTags, InventoryBlock};

pub fn air() -> SimpleBlock {
    SimpleBlock::new("minecraft:air")
}
pub fn white_concrete() -> SimpleBlock {
    SimpleBlock::new("minecraft:white_concrete")
}
pub fn target() -> SimpleBlock {
    SimpleBlock::new("minecraft:target")
}
pub fn upper_smooth_stone_slab() -> BlockWithTags {
    let mut block = BlockWithTags::new("minecraft:smooth_stone_slab");
    block.insert_tag("type", "top");
    block
}
pub fn comperator(facing_direction: Direction, mode: ComperatorMode) -> BlockWithTags {
    let mut block = BlockWithTags::new("minecraft:comparator");
    block.insert_tag("facing", facing_direction);
    block.insert_tag("mode", mode);
    block
}
pub fn repeater(facing_direction: Direction, delay: u16) -> BlockWithTags {
    let mut block = BlockWithTags::new("minecraft:repeater");
    block.insert_tag("facing", facing_direction);
    block.insert_tag("delay", delay.to_string());
    block
}
pub fn redstone_wall_torch(facing_direction: Direction, lit: bool) -> BlockWithTags {
    let mut block = BlockWithTags::new("minecraft:redstone_wall_torch");
    block.insert_tag("facing", facing_direction);
    block.insert_tag("lit", if lit { "true" } else { "false" });
    block
}
pub fn empty_barrel() -> InventoryBlock{
    InventoryBlock::new("minecraft:barrel", 27)
}
pub fn barrel_with_signal_strength(signal_strength: u16) -> Option<InventoryBlock> {
    let mut barrel = empty_barrel();
    barrel.set_signal_strength(signal_strength).ok()?;
    Some(barrel)
}
pub fn furnace_with_signal_strength(signal_strength: u16) -> Option<InventoryBlock> {
    let mut furnace = InventoryBlock::new("minecraft:furnace", 3);
    furnace.set_signal_strength(signal_strength).ok()?;
    Some(furnace)
}
pub fn redstone_wire(connections: Vec<Direction>, power: u8) -> BlockWithTags {
    let mut block = BlockWithTags::new("minecraft:redstone_wire");
    block.insert_tag("power", power.to_string());
    for connected_side in connections {
        block.insert_tag(connected_side, "side");
    }
    block
}
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Into<String> for Direction {
    fn into(self) -> String {
        match self {
            Direction::North => "north".into(),
            Direction::East => "east".into(),
            Direction::South => "south".into(),
            Direction::West => "west".into(),
        }
    }
}
pub enum ComperatorMode {
    Compare,
    Subtract,
}
impl Into<String> for ComperatorMode {
    fn into(self) -> String {
        match self {
            ComperatorMode::Compare => "compare".into(),
            ComperatorMode::Subtract => "subtract".into(),
        }
    }
}

