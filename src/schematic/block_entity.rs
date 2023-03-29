use super::item_slot::ItemSlot;
use serde::Serialize;


#[derive(Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BlockEntity {
    #[serde(serialize_with = "nbt::i32_array")]
    pos: Vec<i32>,
    id: String,
    items: Option<Vec<ItemSlot>>,
    custom_name: Option<String>,
}
impl BlockEntity {
    pub fn new<S: Into<String>>((x, y, z): (i32, i32, i32), id: S) -> BlockEntity {
        Self {
            pos: vec![x, z, y],
            id: id.into(),
            items: None,
            custom_name: None,
        }
    }
    pub fn set_items(&mut self, items: Vec<ItemSlot>) {
        self.items = Some(items);
    }

    pub fn set_custom_name(&mut self, custom_name: String) {
        self.custom_name = Some(custom_name);
    }
}
