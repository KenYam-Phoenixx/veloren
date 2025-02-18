use super::*;
use crate::{
    Land,
    site2::gen::place_circular,
    util::{CARDINALS, NEIGHBORS, RandomField, Sampler},
};
use common::generation::EntityInfo;
use rand::prelude::*;
use std::sync::Arc;
use vek::*;

/// Represents house data generated by the `generate()` method
pub struct MyrmidonHouse {
    /// Axis aligned bounding region for the house
    bounds: Aabr<i32>,
    /// Approximate altitude of the door tile
    pub(crate) alt: i32,
}

impl MyrmidonHouse {
    pub fn generate(
        land: &Land,
        _rng: &mut impl Rng,
        site: &Site,
        tile_aabr: Aabr<i32>,
        alt: Option<i32>,
    ) -> Self {
        let bounds = Aabr {
            min: site.tile_wpos(tile_aabr.min),
            max: site.tile_wpos(tile_aabr.max),
        };
        Self {
            bounds,
            alt: alt.unwrap_or_else(|| {
                land.get_alt_approx(site.tile_center_wpos((tile_aabr.max - tile_aabr.min) / 2))
                    as i32
                    + 2
            }),
        }
    }

    pub fn spawn_rules(&self, wpos: Vec2<i32>) -> SpawnRules {
        SpawnRules {
            waypoints: false,
            trees: wpos.distance_squared(self.bounds.center()) > (85_i32).pow(2),
            ..SpawnRules::default()
        }
    }
}

impl Structure for MyrmidonHouse {
    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"render_myrmidon_house\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "render_myrmidon_house")]
    fn render_inner(&self, _site: &Site, _land: &Land, painter: &Painter) {
        let base = self.alt + 3;
        let center = self.bounds.center();
        let mut thread_rng = thread_rng();
        let sandstone_unbroken = Fill::Sampling(Arc::new(|center| {
            Some(match (RandomField::new(0).get(center)) % 37 {
                0..=8 => Block::new(BlockKind::Rock, Rgb::new(245, 212, 129)),
                9..=17 => Block::new(BlockKind::Rock, Rgb::new(246, 214, 133)),
                18..=26 => Block::new(BlockKind::Rock, Rgb::new(247, 216, 136)),
                27..=35 => Block::new(BlockKind::Rock, Rgb::new(248, 219, 142)),
                _ => Block::new(BlockKind::Rock, Rgb::new(235, 178, 99)),
            })
        }));
        let sandstone = Fill::Sampling(Arc::new(|center| {
            Some(match (RandomField::new(0).get(center)) % 42 {
                0..=8 => Block::new(BlockKind::Rock, Rgb::new(245, 212, 129)),
                9..=17 => Block::new(BlockKind::Rock, Rgb::new(246, 214, 133)),
                18..=26 => Block::new(BlockKind::Rock, Rgb::new(247, 216, 136)),
                27..=35 => Block::new(BlockKind::Rock, Rgb::new(248, 219, 142)),
                36..=37 => Block::new(BlockKind::Air, Rgb::new(0, 0, 0)),
                _ => Block::new(BlockKind::Rock, Rgb::new(235, 178, 99)),
            })
        }));
        let roof_color = Fill::Brick(BlockKind::Sand, Rgb::new(115, 32, 2), 12);
        let diameter =
            (self.bounds.max.x - self.bounds.min.x).min(self.bounds.max.y - self.bounds.min.y);
        let bldg_var = RandomField::new(0).get(center.with_z(base + 5)) % 4;

        if bldg_var < 1 {
            // circle
            let circle_radius = 2 * (diameter / 5);
            painter
                .cylinder(Aabb {
                    min: (center - circle_radius).with_z(base - 30),
                    max: (center + circle_radius).with_z(base - 1),
                })
                .fill(sandstone_unbroken.clone());
            painter
                .cylinder(Aabb {
                    min: (center - circle_radius + 1).with_z(base - 1),
                    max: (center + circle_radius - 1).with_z(base),
                })
                .fill(sandstone.clone());
            painter
                .cylinder(Aabb {
                    min: (center - circle_radius + 2).with_z(base - 1),
                    max: (center + circle_radius - 2).with_z(base + 15),
                })
                .clear();

            for dir in CARDINALS {
                let clear_pos = center + dir * circle_radius;
                let clear_rand = RandomField::new(0).get(clear_pos.with_z(base)) % 2;
                if clear_rand < 1 {
                    painter
                        .cylinder(Aabb {
                            min: (clear_pos - 6).with_z(base - 1),
                            max: (clear_pos + 6).with_z(base),
                        })
                        .clear();
                }
            }
            let pillars = 8 + (RandomField::new(0).get(center.with_z(base + 2)) % 6) as i32;
            let pillar_positions = place_circular(center, (circle_radius - 5) as f32, pillars);
            for pillar in pillar_positions {
                let pillar_rand = RandomField::new(0).get(pillar.with_z(base)) % 5;
                if pillar_rand > 0 {
                    let pillar_heigth = 10 + pillar_rand as i32;
                    painter
                        .cylinder(Aabb {
                            min: (pillar - 3).with_z(base - 1),
                            max: (pillar + 3).with_z(base),
                        })
                        .fill(sandstone.clone());
                    painter
                        .cylinder(Aabb {
                            min: (pillar - 2).with_z(base),
                            max: (pillar + 2).with_z(base + pillar_heigth),
                        })
                        .fill(sandstone.clone());
                    for dir in CARDINALS {
                        let clear_pos = pillar + dir * 3;
                        let clear_var = RandomField::new(0).get(clear_pos.with_z(base)) % 2;

                        if clear_var > 0 {
                            painter
                                .sphere_with_radius(clear_pos.with_z(base + pillar_heigth), 3.0)
                                .clear();
                        }
                    }
                }
            }
        } else {
            // foundation
            painter
                .aabb(Aabb {
                    min: (center - (diameter / 2)).with_z(base - 30),
                    max: (center + (diameter / 2)).with_z(base - 2),
                })
                .fill(sandstone_unbroken.clone());
            // fence
            painter
                .aabb(Aabb {
                    min: Vec2::new(center.x - (diameter / 2), center.y - (diameter / 2))
                        .with_z(base - 2),
                    max: Vec2::new(center.x + (diameter / 2), center.y + (diameter / 2))
                        .with_z(base - 1),
                })
                .fill(sandstone.clone());
            painter
                .aabb(Aabb {
                    min: Vec2::new(center.x - (diameter / 2) + 1, center.y - (diameter / 2) + 1)
                        .with_z(base - 2),
                    max: Vec2::new(center.x + (diameter / 2) - 1, center.y + (diameter / 2) - 1)
                        .with_z(base + 15),
                })
                .clear();
            for dir in CARDINALS {
                let gate_pos = center + dir * (diameter / 2);
                painter
                    .cylinder(Aabb {
                        min: (gate_pos - 6).with_z(base - 2),
                        max: (gate_pos + 6).with_z(base + 6),
                    })
                    .clear();
            }

            // house
            let rand = RandomField::new(0).get(center.with_z(base)) % 2;
            let heigth_rand = (RandomField::new(0).get(center.with_z(base)) % 10) as i32;
            let bldg_length = (diameter / 2) - 5;
            let bldg_width = diameter / 3;
            let bldg_height = (diameter / 4) + heigth_rand;
            let (x_axis, y_axis) = if rand > 0 {
                (bldg_length, bldg_width)
            } else {
                (bldg_width, bldg_length)
            };

            // roof

            painter
                .aabb(Aabb {
                    min: Vec2::new(center.x - x_axis, center.y - y_axis).with_z(base + bldg_height),
                    max: Vec2::new(center.x + x_axis, center.y + y_axis)
                        .with_z(base + bldg_height + 1),
                })
                .fill(sandstone.clone());
            painter
                .aabb(Aabb {
                    min: Vec2::new(center.x - x_axis + 1, center.y - y_axis + 1)
                        .with_z(base + bldg_height + 1),
                    max: Vec2::new(center.x + x_axis - 1, center.y + y_axis - 1)
                        .with_z(base + bldg_height + 2),
                })
                .fill(sandstone.clone());
            painter
                .aabb(Aabb {
                    min: Vec2::new(center.x - x_axis, center.y - y_axis)
                        .with_z(base + bldg_height + 2),
                    max: Vec2::new(center.x + x_axis, center.y + y_axis)
                        .with_z(base + bldg_height + 3),
                })
                .fill(sandstone.clone());
            painter
                .aabb(Aabb {
                    min: Vec2::new(center.x - x_axis + 1, center.y - y_axis + 1)
                        .with_z(base + bldg_height + 3),
                    max: Vec2::new(center.x + x_axis - 1, center.y + y_axis - 1)
                        .with_z(base + bldg_height + 4),
                })
                .fill(sandstone.clone());
            let roof_dir = if rand > 0 { Dir::X } else { Dir::Y };
            painter
                .gable(
                    Aabb {
                        min: Vec2::new(center.x - x_axis, center.y - y_axis)
                            .with_z(base + bldg_height + 4),
                        max: Vec2::new(center.x + x_axis, center.y + y_axis)
                            .with_z(base + bldg_height + 8),
                    },
                    4 * (x_axis / 5),
                    roof_dir,
                )
                .fill(sandstone.clone());

            painter
                .gable(
                    Aabb {
                        min: Vec2::new(center.x - x_axis + 1, center.y - y_axis + 2)
                            .with_z(base + bldg_height + 5),
                        max: Vec2::new(center.x + x_axis - 1, center.y + y_axis - 2)
                            .with_z(base + bldg_height + 9),
                    },
                    4 * (x_axis / 5),
                    roof_dir,
                )
                .fill(roof_color.clone());
            if rand > 0 {
                for r in 0..((x_axis / 2) - 1) {
                    painter
                        .gable(
                            Aabb {
                                min: Vec2::new(
                                    center.x - x_axis + 4 + (4 * r),
                                    center.y - y_axis - 1,
                                )
                                .with_z(base + bldg_height + 6),
                                max: Vec2::new(
                                    center.x - x_axis + 5 + (4 * r),
                                    center.y + y_axis + 1,
                                )
                                .with_z(base + bldg_height + 10),
                            },
                            4 * (x_axis / 5),
                            roof_dir,
                        )
                        .fill(roof_color.clone());
                }
            } else {
                for r in 0..((y_axis / 2) - 1) {
                    painter
                        .gable(
                            Aabb {
                                min: Vec2::new(
                                    center.x - x_axis - 1,
                                    center.y - y_axis + 4 + (4 * r),
                                )
                                .with_z(base + bldg_height + 6),
                                max: Vec2::new(
                                    center.x + x_axis + 1,
                                    center.y - y_axis + 5 + (4 * r),
                                )
                                .with_z(base + bldg_height + 10),
                            },
                            4 * (x_axis / 5),
                            roof_dir,
                        )
                        .fill(roof_color.clone());
                }
            }
            // pillars
            let pillar_x_axis = 3 * (x_axis / 4);
            let pillar_y_axis = 3 * (y_axis / 4);
            for dir in NEIGHBORS {
                let pillar_pos = Vec2::new(
                    center.x + dir.x * pillar_x_axis,
                    center.y + dir.y * pillar_y_axis,
                );

                painter
                    .cylinder(Aabb {
                        min: (pillar_pos - 3).with_z(base - 2),
                        max: (pillar_pos + 3).with_z(base - 1),
                    })
                    .fill(sandstone.clone());
                painter
                    .cylinder(Aabb {
                        min: (pillar_pos - 2).with_z(base - 1),
                        max: (pillar_pos + 2).with_z(base + bldg_height - 2),
                    })
                    .fill(sandstone.clone());
                for p in 0..3 {
                    painter
                        .cylinder(Aabb {
                            min: (pillar_pos - 3 - p).with_z(base + bldg_height - 3 + p),
                            max: (pillar_pos + 3 + p).with_z(base + bldg_height - 2 + p),
                        })
                        .fill(sandstone.clone());
                }

                let decay = (RandomField::new(0).get(pillar_pos.with_z(base)) % 6) as i32;

                if decay < 1 {
                    let decay_rand =
                        12.0 + (RandomField::new(0).get(pillar_pos.with_z(base)) % 6) as f32;

                    painter
                        .sphere_with_radius(pillar_pos.with_z(base + bldg_height + 8), decay_rand)
                        .clear();
                    for dir in CARDINALS {
                        let clear_pos = pillar_pos + dir * 3;
                        let clear_var = RandomField::new(0).get(clear_pos.with_z(base)) % 2;

                        if clear_var > 0 {
                            painter
                                .sphere_with_radius(clear_pos.with_z(base + bldg_height - 4), 3.0)
                                .clear();
                        }
                    }
                }
            }
        }

        // npcs
        let amount = (RandomField::new(0).get(center.with_z(base + 3)) % 5) as i32;
        for n in 0..amount {
            let entities = [
                "common.entity.dungeon.myrmidon.hoplite",
                "common.entity.dungeon.myrmidon.marksman",
                "common.entity.dungeon.myrmidon.strategian",
            ];
            let npc_pos = (center + n).with_z(base);
            let npc = entities[(RandomField::new(0).get(npc_pos) % entities.len() as u32) as usize];
            painter.spawn(EntityInfo::at(npc_pos.as_()).with_asset_expect(
                npc,
                &mut thread_rng,
                None,
            ));
        }
        if amount < 1 {
            if bldg_var > 0 {
                // catacomb
                painter
                    .aabb(Aabb {
                        min: (center - (diameter / 2) + 1).with_z(base - 29),
                        max: (center + (diameter / 2) - 1).with_z(base - 3),
                    })
                    .fill(sandstone.clone());

                for dir in NEIGHBORS {
                    for r in 1..=3 {
                        let room_pos = center + dir * ((diameter / 7) * r);
                        for s in 0..3 {
                            let room_var =
                                RandomField::new(0).get(room_pos.with_z(base + r + s)) % 6;
                            let room_dir = if room_var < 3 { Dir::Y } else { Dir::X };

                            let room = painter.vault(
                                Aabb {
                                    min: (room_pos - (diameter / 10)).with_z(base - 29 + (8 * s)),
                                    max: (room_pos + (diameter / 10)).with_z(base - 23 + (8 * s)),
                                },
                                room_dir,
                            );

                            let chest_var =
                                RandomField::new(0).get(room_pos.with_z(base + r + s)) % 10;

                            match room_var {
                                0 => room.fill(sandstone.clone()),
                                _ => room.clear(),
                            };

                            // storey exit clear
                            if room_var > 3 {
                                let carve_heigth = if s < 2 { 12 } else { 8 };
                                painter
                                    .vault(
                                        Aabb {
                                            min: (room_pos - (diameter / 10))
                                                .with_z(base - 29 + (8 * s)),
                                            max: (room_pos + (diameter / 10))
                                                .with_z(base - 29 + carve_heigth + (8 * s)),
                                        },
                                        room_dir,
                                    )
                                    .clear();
                            }
                            if s < 1 && room_var > 1 && chest_var > 8 {
                                painter
                                    .sprite(room_pos.with_z(base - 29), SpriteKind::DungeonChest4);
                            }
                        }
                    }
                }
                painter
                    .aabb(Aabb {
                        min: (center - (diameter / 10)).with_z(base - 6),
                        max: (center + (diameter / 10)).with_z(base + 2),
                    })
                    .clear();
            }
            let npc_pos = (center - 8).with_z(base);

            painter.spawn(EntityInfo::at(npc_pos.as_()).with_asset_expect(
                "common.entity.dungeon.myrmidon.cyclops",
                &mut thread_rng,
                None,
            ));
        }
    }
}
