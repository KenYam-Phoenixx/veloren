use super::*;
use crate::{
    Land,
    site2::gen::{place_circular_as_vec, spiral_staircase},
    util::{CARDINALS, DIAGONALS, RandomField, Sampler},
};
use common::{
    comp::Content,
    generation::SpecialEntity,
    terrain::{BlockKind, SpriteCfg, SpriteKind},
};
use rand::prelude::*;
use std::{f32::consts::TAU, sync::Arc};
use vek::*;

/// Represents house data generated by the `generate()` method
pub struct SavannahAirshipDock {
    /// Tile position of the door tile
    pub door_tile: Vec2<i32>,
    /// Approximate altitude of the door tile
    pub(crate) alt: i32,
    center: Vec2<i32>,
    length: i32,
    platform_height: i32,
    pub docking_positions: Vec<Vec3<i32>>,
}

impl SavannahAirshipDock {
    pub fn generate(
        land: &Land,
        _rng: &mut impl Rng,
        site: &Site,
        door_tile: Vec2<i32>,
        door_dir: Vec2<i32>,
        tile_aabr: Aabr<i32>,
    ) -> Self {
        let door_tile_pos = site.tile_center_wpos(door_tile);
        let bounds = Aabr {
            min: site.tile_wpos(tile_aabr.min),
            max: site.tile_wpos(tile_aabr.max),
        };
        let center = bounds.center();
        let alt = land.get_alt_approx(site.tile_center_wpos(door_tile + door_dir)) as i32 + 2;
        let base = alt + 1;
        let length = 18;
        let platform_height = 40;
        let mut docking_positions = vec![];
        let top_floor = base + platform_height - 2;
        for dir in CARDINALS {
            let docking_pos = center + dir * (length + 5);
            docking_positions.push(docking_pos.with_z(top_floor));
        }

        Self {
            door_tile: door_tile_pos,
            alt,
            center,
            length,
            platform_height,
            docking_positions,
        }
    }
}

impl Structure for SavannahAirshipDock {
    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"render_savannah_airship_dock\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "render_savannah_airship_dock")]
    fn render_inner(&self, _site: &Site, _land: &Land, painter: &Painter) {
        let base = self.alt + 1;
        let center = self.center;
        let wood_dark = Fill::Brick(BlockKind::Misc, Rgb::new(142, 67, 27), 12);
        let reed = Fill::Brick(BlockKind::Misc, Rgb::new(72, 55, 46), 22);
        let clay = Fill::Brick(BlockKind::Misc, Rgb::new(209, 124, 57), 22);
        let color = Fill::Sampling(Arc::new(|center| {
            Some(match (RandomField::new(0).get(center)) % 7 {
                0 => Block::new(BlockKind::GlowingRock, Rgb::new(153, 82, 40)),
                1 => Block::new(BlockKind::GlowingRock, Rgb::new(172, 104, 57)),
                2 => Block::new(BlockKind::GlowingRock, Rgb::new(135, 106, 100)),
                3 => Block::new(BlockKind::GlowingRock, Rgb::new(198, 164, 139)),
                4 => Block::new(BlockKind::GlowingRock, Rgb::new(168, 163, 157)),
                5 => Block::new(BlockKind::GlowingRock, Rgb::new(73, 53, 42)),
                _ => Block::new(BlockKind::GlowingRock, Rgb::new(178, 124, 90)),
            })
        }));
        let length = self.length;
        let height = length / 2;
        let platform_height = self.platform_height;
        let storeys = 1;
        let radius = length + (length / 3);
        let reed_var = (1 + RandomField::new(0).get(center.with_z(base)) % 4) as f32;
        let reed_parts = 36_f32 + reed_var;
        let phi = TAU / reed_parts;

        // foundation
        painter
            .cylinder(Aabb {
                min: (center - length).with_z(base - 3),
                max: (center + length + 1).with_z(base - 2),
            })
            .fill(clay.clone());
        painter
            .cylinder(Aabb {
                min: (center - length - 1).with_z(base - 4),
                max: (center + length + 2).with_z(base - 3),
            })
            .fill(clay.clone());
        painter
            .cylinder(Aabb {
                min: (center - length - 2).with_z(base - 5),
                max: (center + length + 3).with_z(base - 4),
            })
            .fill(clay.clone());
        painter
            .cylinder(Aabb {
                min: (center - length - 3).with_z(base - height),
                max: (center + length + 4).with_z(base - 5),
            })
            .fill(clay.clone());
        // platform
        painter
            .cylinder(Aabb {
                min: (center - (2 * (length / 3)) - 1).with_z(base + platform_height - 4),
                max: (center + (2 * (length / 3)) + 1).with_z(base + platform_height - 3),
            })
            .fill(color.clone());
        painter
            .cylinder(Aabb {
                min: (center - (2 * (length / 3))).with_z(base + platform_height - 4),
                max: (center + (2 * (length / 3))).with_z(base + platform_height - 3),
            })
            .fill(clay.clone());
        painter
            .cylinder(Aabb {
                min: (center - length - 2).with_z(base + platform_height - 3),
                max: (center + length + 2).with_z(base + platform_height - 2),
            })
            .fill(color.clone());
        painter
            .cylinder(Aabb {
                min: (center - length - 1).with_z(base + platform_height - 3),
                max: (center + length + 1).with_z(base + platform_height - 2),
            })
            .fill(clay.clone());
        // docks
        for dir in CARDINALS {
            let dock_pos = center + dir * (length + 2);
            painter
                .cylinder(Aabb {
                    min: (dock_pos - 5).with_z(base + platform_height - 3),
                    max: (dock_pos + 5).with_z(base + platform_height - 2),
                })
                .fill(color.clone());
            painter
                .cylinder(Aabb {
                    min: (dock_pos - 4).with_z(base + platform_height - 3),
                    max: (dock_pos + 4).with_z(base + platform_height - 2),
                })
                .fill(wood_dark.clone());
        }

        for dock_pos in &self.docking_positions {
            painter.rotated_sprite_with_cfg(
                *dock_pos,
                SpriteKind::Sign,
                Dir::from_vec2(dock_pos.xy() - self.center).sprite_ori(),
                SpriteCfg {
                    unlock: None,
                    content: Some(Content::localized("common-signs-airship_dock")),
                },
            );
        }

        // lanterns, crates & barrels
        for dir in CARDINALS {
            let lantern_pos = center + (dir * length);

            painter.sprite(
                lantern_pos.with_z(base + platform_height - 2),
                SpriteKind::Lantern,
            );
        }
        for dir in DIAGONALS {
            let cargo_pos = center + (dir * ((length / 2) - 1));
            for dir in CARDINALS {
                let sprite_pos = cargo_pos + dir;
                let rows = (RandomField::new(0).get(sprite_pos.with_z(base)) % 3) as i32;
                for r in 0..rows {
                    painter
                        .aabb(Aabb {
                            min: (sprite_pos).with_z(base + platform_height - 2 + r),
                            max: (sprite_pos + 1).with_z(base + platform_height - 1 + r),
                        })
                        .fill(Fill::Block(Block::air(
                            match (RandomField::new(0).get(sprite_pos.with_z(base + r)) % 2) as i32
                            {
                                0 => SpriteKind::Barrel,
                                _ => SpriteKind::CrateBlock,
                            },
                        )));
                    if r > 0 {
                        painter.owned_resource_sprite(
                            sprite_pos.with_z(base + platform_height - 1 + r),
                            SpriteKind::Crate,
                            0,
                        );
                    }
                }
            }
        }
        // campfire
        let campfire_pos = (center - (2 * (length / 3)) - 1).with_z(base + platform_height);
        painter.spawn(
            EntityInfo::at(campfire_pos.map(|e| e as f32 + 0.5))
                .into_special(SpecialEntity::Waypoint),
        );
        for b in 0..2 {
            let base = base + (b * platform_height);
            let radius = radius - (b * (radius / 3));
            let length = length - (b * (length / 3));
            // roof cone
            painter
                .cone(Aabb {
                    min: (center - radius).with_z(base + (storeys * height) - (height / 2) + 1),
                    max: (center + radius)
                        .with_z(base + (storeys * height) + (height / 2) - 1 + reed_var as i32),
                })
                .fill(reed.clone());
            painter
                .cone(Aabb {
                    min: (center - radius).with_z(base + (storeys * height) - (height / 2)),
                    max: (center + radius)
                        .with_z(base + (storeys * height) + (height / 2) - 2 + reed_var as i32),
                })
                .clear();
            // room
            for s in 0..storeys {
                let room = painter.cylinder(Aabb {
                    min: (center - length + 2 + s).with_z(base - 2 + (s * height)),
                    max: (center + 1 + length - 2 - s).with_z(base + height + (s * height)),
                });
                room.fill(clay.clone());
                // decor inlays
                for dir in DIAGONALS {
                    let decor_pos = center + dir * (length - 2 - s);
                    let decor = painter
                        .line(
                            center.with_z(base - 1 + (s * (height + 2))),
                            decor_pos.with_z(base - 1 + (s * (height + 2))),
                            5.0,
                        )
                        .intersect(room);
                    decor.fill(color.clone());
                    painter
                        .line(
                            center.with_z(base - 1 + (s * (height + 2))),
                            decor_pos.with_z(base - 1 + (s * (height + 2))),
                            4.0,
                        )
                        .intersect(decor)
                        .fill(clay.clone());
                }
            }
            // clear rooms
            painter
                .cylinder(Aabb {
                    min: (center - length + 4).with_z(base - 2),
                    max: (center + 1 + length - 4).with_z(base + (storeys * height)),
                })
                .clear();
            // wood decor
            painter
                .cylinder(Aabb {
                    min: (center - length + 4).with_z(base - 1),
                    max: (center + 1 + length - 4).with_z(base),
                })
                .fill(wood_dark.clone());
            painter
                .cylinder(Aabb {
                    min: (center - length + 4).with_z(base + (storeys * height) - 1),
                    max: (center + 1 + length - 4).with_z(base + (storeys * height) + 1),
                })
                .fill(wood_dark.clone());

            for s in 0..storeys {
                // entries, windows
                for dir in CARDINALS {
                    let frame_pos = center + dir * (length - 2 - s);
                    let clear_pos = center + dir * (length + 2 - s);

                    painter
                        .line(
                            center.with_z(base - 1 + (s * (height + 2))),
                            frame_pos.with_z(base - 1 + (s * (height + 2))),
                            3.0,
                        )
                        .fill(color.clone());
                    painter
                        .line(
                            center.with_z(base - 1 + (s * (height + 2))),
                            clear_pos.with_z(base - 1 + (s * (height + 2))),
                            2.0,
                        )
                        .clear();
                }
            }
            // re clear room
            painter
                .cylinder(Aabb {
                    min: (center - length + 5).with_z(base - 2),
                    max: (center + 1 + length - 5).with_z(base + (storeys * height) + 1),
                })
                .clear();
            // floor
            painter
                .cylinder(Aabb {
                    min: (center - (length / 2) - 1).with_z(base - 3),
                    max: (center + (length / 2) + 1).with_z(base - 2),
                })
                .fill(color.clone());
            painter
                .cylinder(Aabb {
                    min: (center - (length / 2) + 1).with_z(base - 3),
                    max: (center + (length / 2) - 1).with_z(base - 2),
                })
                .fill(clay.clone());

            // reed roof lines
            for n in 1..=reed_parts as i32 {
                let pos = Vec2::new(
                    center.x + ((radius as f32) * ((n as f32 * phi).cos())) as i32,
                    center.y + ((radius as f32) * ((n as f32 * phi).sin())) as i32,
                );
                painter
                    .line(
                        pos.with_z(base + (storeys * height) - (height / 2)),
                        center.with_z(base + (storeys * height) + (height / 2) + reed_var as i32),
                        1.0,
                    )
                    .fill(reed.clone());
            }
        }

        // tower
        let beams_low = place_circular_as_vec(center, (2 * (length / 3)) as f32, 10);
        let beams_high = place_circular_as_vec(center, (2 * (length / 4)) as f32, 10);

        for b in 0..beams_low.len() {
            painter
                .cylinder(Aabb {
                    min: (beams_low[b] - 4).with_z(base + height - 1),
                    max: (beams_low[b] + 4).with_z(base + height),
                })
                .fill(wood_dark.clone());

            painter
                .line(
                    beams_low[b].with_z(base + height),
                    beams_high[b].with_z(base + platform_height - 4),
                    1.5,
                )
                .fill(wood_dark.clone());
        }
        //stairs
        painter
            .cylinder(Aabb {
                min: (center - (length / 3)).with_z(base),
                max: (center + (length / 3)).with_z(base + platform_height),
            })
            .clear();

        let stairs = painter.cylinder(Aabb {
            min: (center - (length / 3)).with_z(base - 3),
            max: (center + (length / 3)).with_z(base + platform_height - 2),
        });

        stairs
            .sample(spiral_staircase(
                center.with_z(base - 3),
                ((length / 3) + 1) as f32,
                0.5,
                (platform_height / 4) as f32,
            ))
            .fill(clay.clone());
    }
}
