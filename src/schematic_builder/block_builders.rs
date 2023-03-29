use itertools::Itertools;
use nbt::Map;

use crate::schematic::{block_entity::BlockEntity, item_slot::ItemSlot};
use super::{schematic_builder::SchematicBuilder, Pos};

pub trait BlockBuilder {
    fn get_tag(&self, pos: Pos, sb: &SchematicBuilder) -> String;
    #[allow(unused_variables)]
    fn get_block_entity(&self, pos: Pos, sb: &SchematicBuilder) -> Option<BlockEntity>{
        None
    }
}
#[derive(Clone)]
pub struct SimpleBlock {
    name: String,
}
impl SimpleBlock {
    pub fn new<S: Into<String>>(s: S) -> SimpleBlock {
        SimpleBlock { name: s.into() }
    }
}
impl BlockBuilder for SimpleBlock {
    fn get_tag(&self, _pos: Pos, _sb: &SchematicBuilder) -> String {
        self.name.clone()
    }
}
pub struct BlockWithTags{
    name: String,
    tags: Map<String, String>
}

impl BlockWithTags {
    pub fn insert_tag(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.tags.insert(key.into(), value.into());
    }
    pub fn new(name: impl Into<String>) -> BlockWithTags {
        return BlockWithTags {
            name: name.into(),
            tags: Map::new(),
        };
    }
}
impl BlockBuilder for BlockWithTags {
    fn get_tag(&self, _: Pos, _: &SchematicBuilder) -> String {
        generate_tag(self.name.clone(), &self.tags)
    }
}


#[derive(Clone, Debug)]
pub struct InventoryBlock {
    name: String,
    tags: Map<String, String>,
    slots: u16,
    items: Vec<ItemSlot>,
    custom_name: Option<String>,
}
impl InventoryBlock {
    pub fn new(name: impl Into<String>, slots: u16) -> InventoryBlock{
        InventoryBlock {
            name: name.into(),
            tags: Map::new(),
            slots,
            items: vec![],
            custom_name: None,
        }
    }


    pub fn set_signal_strength(&mut self, signal_strength: u16) -> Result<(), ()> {
        self.items = ItemSlot::get_items_for_signal_strength(
            self.slots,
            "minecraft:totem_of_undying",
            "minecraft:redstone",
            signal_strength,
        )
        .ok_or(())?;
        self.custom_name = Some(format!("{{\"text\":\"§4§l{}\"}}", signal_strength));
        Ok(())
    }

    pub fn get_items(&self) -> &Vec<ItemSlot> {
        &self.items
    }
    pub fn get_items_mut(&mut self) -> &mut Vec<ItemSlot>{
        &mut self.items
    }
    pub fn set_items(&mut self, items: Vec<ItemSlot>){
        self.items = items;
    } 
}
impl BlockBuilder for InventoryBlock {
    fn get_tag(&self, _pos: Pos, _sb: &SchematicBuilder) -> String {
        generate_tag(self.name.clone(), &self.tags)
    }
    fn get_block_entity(&self, pos: Pos, _sb: &SchematicBuilder) -> Option<BlockEntity> {
        let mut block_entity = BlockEntity::new(pos, self.name.clone());

        block_entity.set_items(self.items.clone());

        if let Some(custom_name) = self.custom_name.clone() {
            block_entity.set_custom_name(custom_name);
        }

        Some(block_entity)
    }
}


fn generate_tag(mut name: String, tags: &Map<String, String>) -> String {
    if tags.len() == 0 {
        name
    } else {
        name.push('[');
        name.push_str(
            tags.iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .join(",")
                .as_str(),
        );
        name.push(']');
        name
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_generate_tag(){
        let name = "name".to_string();
        let mut tags = Map::new();
        tags.insert("key1".to_string(), "tag1".to_string());

        assert_eq!("name[key1=tag1]", generate_tag(name, &tags));
    }
    #[test]
    fn test_generate_tag_no_tags(){
        let name = "name".to_string();
        let tags = Map::new();

        assert_eq!("name", generate_tag(name, &tags));
    }
}