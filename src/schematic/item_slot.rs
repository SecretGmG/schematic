use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ItemSlot {
    id: String,
    #[serde(rename = "Slot")]
    slot: i8,
    #[serde(rename = "Count")]
    count: i8,
}
#[allow(dead_code)]
impl ItemSlot {

    pub fn new<S: Into<String>>(name: S, slot: i8, count: i8) -> ItemSlot{
        ItemSlot { id: name.into(), slot: slot, count: count }
    }

    pub fn stone_item(slot: i8, count: i8) -> ItemSlot {
        ItemSlot::new("minecraft:stone", slot, count)
    }

    pub fn get_items_for_signal_strength<S1: Into<String> + Copy, S2: Into<String> + Copy>(
        slots: u16,
        unstackable: S1,
        stackable: S2,
        signal_strength: u16,
    ) -> Option<Vec<ItemSlot>> {
        if signal_strength == 0 {
            return Some(vec![]);
        }
        if signal_strength == 1 {
            return Some(vec![ItemSlot {
                slot: 0,
                id: stackable.into(),
                count: 1,
            }]);
        }

        //signal_strength = floor(1+inventory_fullnes * 14)
        //inventory_fullness = (signal_strength-1)/14

        let min_inventory_fullness = (signal_strength as f64 - 1.0) / 14.0;
        let max_inventory_fullness = (signal_strength as f64) / 14.0;

        let slots_lower_bound = min_inventory_fullness * slots as f64;
        let slots_upper_bound = max_inventory_fullness * slots as f64;

        let stacks;
        let rest;

        //if the signal strength is representable without any stackable items, then do so
        if (slots_lower_bound).ceil() <= (slots_upper_bound).ceil() - 1.0 {
            stacks = slots_lower_bound.ceil() as u16;
            rest = 0;
        } else {
            stacks = slots_lower_bound as u16;
            rest = (slots_upper_bound * 64.0).ceil() as u16 % 64;
        }

        let mut items = vec![];
        //if the signal strength is representable without stacking unstackable items, then do so
        let needed_slots = stacks + if rest == 0 { 0 } else { 1 };
        if needed_slots <= slots {
            for i in 0..stacks {
                items.push(ItemSlot {
                    id: unstackable.into(),
                    slot: i as i8,
                    count: 1,
                });
            }
            if rest != 0 {
                items.push(ItemSlot {
                    id: stackable.into(),
                    slot: stacks as i8,
                    count: rest as i8,
                });
            }
        }
        //if you need unstackable items stack them up to 64
        else {
            let mut slot = 0;
            for _ in 0..stacks / 64 {
                items.push(ItemSlot {
                    id: unstackable.into(),
                    slot,
                    count: 64,
                });
                slot += 1;
            }
            let stack_rest = stacks % 64;
            if stack_rest != 0 {
                items.push(ItemSlot {
                    id: unstackable.into(),
                    slot,
                    count: stack_rest as i8,
                });
                slot += 1;
            }
            if rest != 0{
                items.push(ItemSlot {
                    id: stackable.into(),
                    slot,
                    count: rest as i8,
                });
            }
        }
        if items.len() as u16 > slots {
            return None;
        } else {
            Some(items)
        }
    }
}
