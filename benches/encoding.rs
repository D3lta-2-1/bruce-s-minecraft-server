use std::str::Bytes;

use common_server::encoding::Encoder;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minecraft::protocol::v1_20_4::registry::{RegistryData, Registry, RegistryEntry, ChatType, Decoration, Biome, Effects, DimensionType, MonsterSpawnLightLevel, IntegerDistribution};

fn encode_registry() {
    let registry_data = RegistryData {
        chat_type_registry: Registry {
            name: "minecraft:chat_type".to_string(),
            value: vec![RegistryEntry {
                name: "minecraft:chat".to_string(),
                id: 0,
                element: ChatType {
                    chat: Decoration {
                        translation_key: "chat.type.text".to_string(),
                        style: None,
                        parameters: vec!["sender".to_string(), "content".to_string()],
                    },
                    narration: Decoration {
                        translation_key: "chat.type.text.narrate".to_string(),
                        style: None,
                        parameters: vec!["sender".to_string(), "content".to_string()],
                    },
                },
            }],
        },
        biome_registry: Registry {
            name: "minecraft:worldgen/biome".to_string(),
            value: vec![RegistryEntry {
                name: "minecraft:plains".to_string(),
                id: 0,
                element: Biome {
                    has_precipitation: false,
                    temperature: 0.5,
                    temperature_modifier: None,
                    downfall: 0.5,
                    effects: Effects {
                        fog_color: 12638463,
                        water_color: 4159204,
                        water_fog_color: 329011,
                        sky_color: 8103167,
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        particle: None,
                        ambient_sound: None,
                        mood_sound: None,
                        additions_sound: None,
                        music: None,
                    },
                },
            }],
        },
        dimension_type_registry: Registry {
            name: "minecraft:dimension_type".to_string(),
            value: vec![RegistryEntry {
                name: "minecraft:overworld".to_string(),
                id: 0,
                element: DimensionType {
                    fixed_type: None,
                    has_skylight: true,
                    has_ceiling: false,
                    ultrawarm: false,
                    natural: true,
                    coordinate_scale: 1.0,
                    bed_works: true,
                    respawn_anchor_works: false,
                    min_y: -64,
                    height: 384,
                    logical_height: 384,
                    infiniburn: "#minecraft:infiniburn_overworld".to_string(),
                    effects: "minecraft:overworld".to_string(),
                    ambient_light: 0.0,
                    piglin_safe: false,
                    has_raids: true,
                    monster_spawn_light_level: MonsterSpawnLightLevel {
                        name: "minecraft:uniform".to_string(),
                        value: IntegerDistribution {
                            min_inclusive: 0,
                            max_inclusive: 7,
                        },
                    },
                    monster_spawn_block_light_limit: 0,
                },
            }],
        },
    };
    registry_data.encode().unwrap();
}

fn encoding(c: &mut Criterion) {
    c.bench_function("encode registry", |b| b.iter(|| encode_registry()));
}

criterion_group!(benches, encoding);
criterion_main!(benches);
