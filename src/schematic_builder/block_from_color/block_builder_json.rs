use super::{BlockColor, BlockSelector};

pub(super) fn get_block_selector() -> BlockSelector{
    let blocks: Vec<BlockColor> = serde_json::from_str(JSON).unwrap();
    return BlockSelector::from_vec(blocks);
}

const JSON: &str = r#"[
    {
        "name": "acacia_planks",
        "r": 168,
        "g": 90,
        "b": 50
    },
    {
        "name": "andesite",
        "r": 136,
        "g": 136,
        "b": 136
    },
    {
        "name": "birch_planks",
        "r": 192,
        "g": 175,
        "b": 121
    },
    {
        "name": "black_glazed_terracotta",
        "r": 67,
        "g": 30,
        "b": 32
    },
    {
        "name": "black_stained_glass",
        "r": 25,
        "g": 25,
        "b": 25
    },
    {
        "name": "black_terracotta",
        "r": 37,
        "g": 22,
        "b": 16
    },
    {
        "name": "black_wool",
        "r": 20,
        "g": 21,
        "b": 25
    },
    {
        "name": "blue_glazed_terracotta",
        "r": 47,
        "g": 64,
        "b": 139
    },
    {
        "name": "blue_ice",
        "r": 116,
        "g": 167,
        "b": 253
    },
    {
        "name": "blue_stained_glass",
        "r": 51,
        "g": 76,
        "b": 178
    },
    {
        "name": "blue_terracotta",
        "r": 74,
        "g": 59,
        "b": 91
    },
    {
        "name": "blue_wool",
        "r": 53,
        "g": 57,
        "b": 157
    },
    {
        "name": "bricks",
        "r": 150,
        "g": 97,
        "b": 83
    },
    {
        "name": "brown_glazed_terracotta",
        "r": 119,
        "g": 106,
        "b": 85
    },
    {
        "name": "brown_mushroom_block",
        "r": 149,
        "g": 111,
        "b": 81
    },
    {
        "name": "brown_stained_glass",
        "r": 102,
        "g": 76,
        "b": 51
    },
    {
        "name": "brown_terracotta",
        "r": 77,
        "g": 51,
        "b": 35
    },
    {
        "name": "brown_wool",
        "r": 114,
        "g": 71,
        "b": 40
    },
    {
        "name": "chiseled_nether_bricks",
        "r": 47,
        "g": 23,
        "b": 28
    },
    {
        "name": "chiseled_quartz_block",
        "r": 231,
        "g": 226,
        "b": 218
    },
    {
        "name": "clay",
        "r": 160,
        "g": 166,
        "b": 179
    },
    {
        "name": "coal_block",
        "r": 16,
        "g": 15,
        "b": 15
    },
    {
        "name": "coal_ore",
        "r": 116,
        "g": 116,
        "b": 116
    },
    {
        "name": "coarse_dirt",
        "r": 119,
        "g": 85,
        "b": 59
    },
    {
        "name": "cracked_nether_bricks",
        "r": 40,
        "g": 20,
        "b": 23
    },
    {
        "name": "crying_obsidian",
        "r": 32,
        "g": 10,
        "b": 60
    },
    {
        "name": "cyan_glazed_terracotta",
        "r": 52,
        "g": 118,
        "b": 125
    },
    {
        "name": "cyan_stained_glass",
        "r": 76,
        "g": 127,
        "b": 153
    },
    {
        "name": "cyan_terracotta",
        "r": 86,
        "g": 91,
        "b": 91
    },
    {
        "name": "cyan_wool",
        "r": 21,
        "g": 137,
        "b": 145
    },
    {
        "name": "dark_oak_log",
        "r": 60,
        "g": 46,
        "b": 26
    },
    {
        "name": "dark_oak_planks",
        "r": 66,
        "g": 43,
        "b": 20
    },
    {
        "name": "dark_prismarine",
        "r": 51,
        "g": 91,
        "b": 75
    },
    {
        "name": "diorite",
        "r": 188,
        "g": 188,
        "b": 188
    },
    {
        "name": "dirt",
        "r": 134,
        "g": 96,
        "b": 67
    },
    {
        "name": "emerald_block",
        "r": 42,
        "g": 203,
        "b": 87
    },
    {
        "name": "emerald_ore",
        "r": 117,
        "g": 136,
        "b": 124
    },
    {
        "name": "glass",
        "r": 175,
        "g": 213,
        "b": 219
    },
    {
        "name": "gold_block",
        "r": 246,
        "g": 208,
        "b": 61
    },
    {
        "name": "gold_ore",
        "r": 143,
        "g": 140,
        "b": 125
    },
    {
        "name": "granite",
        "r": 149,
        "g": 103,
        "b": 85
    },
    {
        "name": "gray_glazed_terracotta",
        "r": 83,
        "g": 90,
        "b": 93
    },
    {
        "name": "gray_stained_glass",
        "r": 76,
        "g": 76,
        "b": 76
    },
    {
        "name": "gray_terracotta",
        "r": 57,
        "g": 42,
        "b": 35
    },
    {
        "name": "gray_wool",
        "r": 62,
        "g": 68,
        "b": 71
    },
    {
        "name": "green_glazed_terracotta",
        "r": 117,
        "g": 142,
        "b": 67
    },
    {
        "name": "green_stained_glass",
        "r": 102,
        "g": 127,
        "b": 51
    },
    {
        "name": "green_terracotta",
        "r": 76,
        "g": 83,
        "b": 42
    },
    {
        "name": "green_wool",
        "r": 84,
        "g": 109,
        "b": 27
    },
    {
        "name": "jungle_planks",
        "r": 160,
        "g": 115,
        "b": 80
    },
    {
        "name": "lapis_block",
        "r": 30,
        "g": 67,
        "b": 140
    },
    {
        "name": "lapis_ore",
        "r": 99,
        "g": 110,
        "b": 132
    },
    {
        "name": "light_blue_glazed_terracotta",
        "r": 94,
        "g": 164,
        "b": 208
    },
    {
        "name": "light_blue_stained_glass",
        "r": 102,
        "g": 153,
        "b": 216
    },
    {
        "name": "light_blue_terracotta",
        "r": 113,
        "g": 108,
        "b": 137
    },
    {
        "name": "light_blue_wool",
        "r": 58,
        "g": 175,
        "b": 217
    },
    {
        "name": "light_gray_glazed_terracotta",
        "r": 144,
        "g": 166,
        "b": 167
    },
    {
        "name": "light_gray_stained_glass",
        "r": 153,
        "g": 153,
        "b": 153
    },
    {
        "name": "light_gray_terracotta",
        "r": 135,
        "g": 106,
        "b": 97
    },
    {
        "name": "light_gray_wool",
        "r": 142,
        "g": 142,
        "b": 134
    },
    {
        "name": "lime_glazed_terracotta",
        "r": 162,
        "g": 197,
        "b": 55
    },
    {
        "name": "lime_stained_glass",
        "r": 127,
        "g": 204,
        "b": 25
    },
    {
        "name": "lime_terracotta",
        "r": 103,
        "g": 117,
        "b": 52
    },
    {
        "name": "lime_wool",
        "r": 112,
        "g": 185,
        "b": 25
    },
    {
        "name": "magenta_glazed_terracotta",
        "r": 208,
        "g": 100,
        "b": 191
    },
    {
        "name": "magenta_stained_glass",
        "r": 178,
        "g": 76,
        "b": 216
    },
    {
        "name": "magenta_terracotta",
        "r": 149,
        "g": 88,
        "b": 108
    },
    {
        "name": "magenta_wool",
        "r": 189,
        "g": 68,
        "b": 179
    },
    {
        "name": "nether_bricks",
        "r": 0,
        "g": 0,
        "b": 3
    },
    {
        "name": "nether_gold_ore",
        "r": 115,
        "g": 54,
        "b": 42
    },
    {
        "name": "nether_quartz_ore",
        "r": 117,
        "g": 65,
        "b": 62
    },
    {
        "name": "nether_wart_block",
        "r": 114,
        "g": 2,
        "b": 2
    },
    {
        "name": "netherite_block",
        "r": 66,
        "g": 61,
        "b": 63
    },
    {
        "name": "netherrack",
        "r": 97,
        "g": 38,
        "b": 38
    },
    {
        "name": "note_block",
        "r": 88,
        "g": 58,
        "b": 40
    },
    {
        "name": "oak_log",
        "r": 109,
        "g": 85,
        "b": 50
    },
    {
        "name": "oak_planks",
        "r": 162,
        "g": 130,
        "b": 78
    },
    {
        "name": "obsidian",
        "r": 15,
        "g": 10,
        "b": 24
    },
    {
        "name": "orange_glazed_terracotta",
        "r": 154,
        "g": 147,
        "b": 91
    },
    {
        "name": "orange_stained_glass",
        "r": 216,
        "g": 127,
        "b": 51
    },
    {
        "name": "orange_terracotta",
        "r": 161,
        "g": 83,
        "b": 37
    },
    {
        "name": "orange_wool",
        "r": 240,
        "g": 118,
        "b": 19
    },
    {
        "name": "packed_ice",
        "r": 141,
        "g": 180,
        "b": 250
    },
    {
        "name": "pink_glazed_terracotta",
        "r": 235,
        "g": 154,
        "b": 181
    },
    {
        "name": "pink_stained_glass",
        "r": 242,
        "g": 127,
        "b": 165
    },
    {
        "name": "pink_terracotta",
        "r": 161,
        "g": 78,
        "b": 78
    },
    {
        "name": "pink_wool",
        "r": 237,
        "g": 141,
        "b": 172
    },
    {
        "name": "polished_andesite",
        "r": 132,
        "g": 134,
        "b": 133
    },
    {
        "name": "polished_diorite",
        "r": 192,
        "g": 193,
        "b": 194
    },
    {
        "name": "polished_granite",
        "r": 154,
        "g": 106,
        "b": 89
    },
    {
        "name": "prismarine",
        "r": 99,
        "g": 156,
        "b": 151
    },
    {
        "name": "prismarine_bricks",
        "r": 99,
        "g": 171,
        "b": 158
    },
    {
        "name": "purple_glazed_terracotta",
        "r": 109,
        "g": 48,
        "b": 152
    },
    {
        "name": "purple_stained_glass",
        "r": 127,
        "g": 63,
        "b": 178
    },
    {
        "name": "purple_terracotta",
        "r": 118,
        "g": 70,
        "b": 86
    },
    {
        "name": "purple_wool",
        "r": 121,
        "g": 42,
        "b": 172
    },
    {
        "name": "purpur_block",
        "r": 169,
        "g": 125,
        "b": 169
    },
    {
        "name": "purpur_pillar",
        "r": 171,
        "g": 129,
        "b": 171
    },
    {
        "name": "quartz_bricks",
        "r": 234,
        "g": 229,
        "b": 221
    },
    {
        "name": "quartz_pillar",
        "r": 235,
        "g": 230,
        "b": 224
    },
    {
        "name": "red_glazed_terracotta",
        "r": 181,
        "g": 59,
        "b": 53
    },
    {
        "name": "red_mushroom_block",
        "r": 200,
        "g": 46,
        "b": 45
    },
    {
        "name": "red_nether_bricks",
        "r": 69,
        "g": 7,
        "b": 9
    },
    {
        "name": "red_stained_glass",
        "r": 153,
        "g": 51,
        "b": 51
    },
    {
        "name": "red_terracotta",
        "r": 143,
        "g": 61,
        "b": 46
    },
    {
        "name": "red_wool",
        "r": 160,
        "g": 39,
        "b": 34
    },
    {
        "name": "soul_sand",
        "r": 81,
        "g": 62,
        "b": 50
    },
    {
        "name": "soul_soil",
        "r": 75,
        "g": 57,
        "b": 46
    },
    {
        "name": "spruce_log",
        "r": 58,
        "g": 37,
        "b": 16
    },
    {
        "name": "spruce_planks",
        "r": 114,
        "g": 84,
        "b": 48
    },
    {
        "name": "stripped_acacia_log",
        "r": 174,
        "g": 92,
        "b": 59
    },
    {
        "name": "stripped_birch_log",
        "r": 196,
        "g": 176,
        "b": 118
    },
    {
        "name": "stripped_dark_oak_log",
        "r": 96,
        "g": 76,
        "b": 49
    },
    {
        "name": "stripped_jungle_log",
        "r": 171,
        "g": 132,
        "b": 84
    },
    {
        "name": "stripped_oak_log",
        "r": 177,
        "g": 144,
        "b": 86
    },
    {
        "name": "stripped_spruce_log",
        "r": 115,
        "g": 89,
        "b": 52
    },
    {
        "name": "terracotta",
        "r": 152,
        "g": 94,
        "b": 67
    },
    {
        "name": "warped_planks",
        "r": 43,
        "g": 104,
        "b": 99
    },
    {
        "name": "warped_wart_block",
        "r": 22,
        "g": 119,
        "b": 121
    },
    {
        "name": "white_glazed_terracotta",
        "r": 188,
        "g": 212,
        "b": 202
    },
    {
        "name": "white_stained_glass",
        "r": 255,
        "g": 255,
        "b": 255
    },
    {
        "name": "white_terracotta",
        "r": 209,
        "g": 178,
        "b": 161
    },
    {
        "name": "white_wool",
        "r": 233,
        "g": 236,
        "b": 236
    },
    {
        "name": "yellow_glazed_terracotta",
        "r": 234,
        "g": 192,
        "b": 88
    },
    {
        "name": "yellow_stained_glass",
        "r": 229,
        "g": 229,
        "b": 51
    },
    {
        "name": "yellow_terracotta",
        "r": 186,
        "g": 133,
        "b": 35
    },
    {
        "name": "yellow_wool",
        "r": 248,
        "g": 197,
        "b": 39
    }
]"#;
