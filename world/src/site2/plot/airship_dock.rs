use super::*;
use crate::{
    site2::gen::{spiral_staircase, PrimitiveTransform},
    util::{RandomField, Sampler, CARDINALS},
    Land,
    site2::gen::{PrimitiveTransform, spiral_staircase},
    util::{RandomField, Sampler},
};
use common::{
    generation::SpecialEntity,
    terrain::{BlockKind, SpriteKind},
};
use rand::prelude::*;
use std::f32::consts::PI;
use vek::*;
/// Represents house data generated by the `generate()` method
pub struct AirshipDock {
    /// Approximate altitude of the door tile
    pub(crate) alt: i32,
    rotation: f32,
    pub door_tile: Vec2<i32>,
    pub center: Vec2<i32>,
    base: i32,
    height: i32,
    pub docking_positions: Vec<Vec3<i32>>,
    pub door_dir: Vec2<i32>,
    campfire_pos: Vec3<i32>,
}

impl AirshipDock {
    pub fn generate(
        land: &Land,
        _rng: &mut impl Rng,
        site: &Site,
        door_tile: Vec2<i32>,
        door_dir: Vec2<i32>,
        tile_aabr: Aabr<i32>,
    ) -> Self {
        let door_tile_pos: Vec2<i32> = site.tile_center_wpos(door_tile);
        let bounds = Aabr {
            min: site.tile_wpos(tile_aabr.min),
            max: site.tile_wpos(tile_aabr.max),
        };
        let center = bounds.center();
        let alt = land.get_alt_approx(site.tile_center_wpos(door_tile + door_dir)) as i32;
        let base = alt + 3;
        let height = base + 28;
        let rotation = if door_dir.y < 0 {
            PI
        } else if door_dir.x < 0 {
            PI / 2.0
        } else if door_dir.y > 0 {
            3.0 * PI / 2.0
        } else {
            0.0
        };
        let mut docking_positions = vec![];
        for dir in CARDINALS {
            let pos = (center + dir * 17).with_z(height + 9);
            docking_positions.push(pos);
        }
        let campfire_pos = (center - (door_dir * 10)).with_z(height + 9);
        Self {
            door_tile: door_tile_pos,
            alt,
            rotation,
            center,
            base,
            height,
            docking_positions,
            door_dir,
            campfire_pos,
        }
    }

    pub fn spawn_rules(&self, wpos: Vec2<i32>) -> SpawnRules {
        SpawnRules {
            trees: {
                // dock is 3 tiles = 18 blocks in radius
                // airships are 20 blocks wide.
                // Leave extra space for tree width (at lease 15 extra).
                // Don't allow trees within 18 + 20 + 15 = 53 blocks of the dock center
                const AIRSHIP_MIN_TREE_DIST2: i32 = 53i32.pow(2);
                wpos.distance_squared(self.center) > AIRSHIP_MIN_TREE_DIST2
            },
            waypoints: false,
            ..SpawnRules::default()
        }
    }
}

impl Structure for AirshipDock {
    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"render_airshipdock\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "render_airshipdock")]
    fn render_inner(&self, _site: &Site, _land: &Land, painter: &Painter) {
        let brick = Fill::Brick(BlockKind::Rock, Rgb::new(80, 75, 85), 24);
        let wood = Fill::Brick(BlockKind::Rock, Rgb::new(45, 28, 21), 24);
        let woodalt = Fill::Brick(BlockKind::Rock, Rgb::new(30, 22, 15), 24);

        let base = self.base;
        let center = self.center;
        let height = self.height;

        //lower doorway
        painter
            .cylinder_with_radius(
                Vec2::new(center.x - 1, center.y + 12).with_z(base - 5),
                4.5,
                7.0,
            )
            .rotate_about_min(Mat3::new(1, 0, 0, 0, 0, -1, 0, 1, 0))
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());

        //bracing
        painter
            .cylinder_with_radius(center.with_z(height), 7.0, 7.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .cylinder_with_radius(center.with_z(height + 1), 7.0, 5.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        painter
            .cylinder_with_radius(center.with_z(height + 7), 8.0, 1.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());

        //platform edging
        painter
            .superquadric(
                Aabb {
                    min: Vec2::new(center.x - 13, center.y - 16).with_z(base + 35),
                    max: Vec2::new(center.x + 17, center.y + 11).with_z(height + 11),
                },
                5.0,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(woodalt.clone());
        //platform
        painter
            .superquadric(
                Aabb {
                    min: Vec2::new(center.x - 12, center.y - 15).with_z(base + 36),
                    max: Vec2::new(center.x + 16, center.y + 10).with_z(height + 11),
                },
                5.0,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .superquadric(
                Aabb {
                    min: Vec2::new(center.x - 12, center.y - 15).with_z(base + 37),
                    max: Vec2::new(center.x + 16, center.y + 10).with_z(height + 12),
                },
                5.0,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        //platform walkway bits
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 2, center.y - 22).with_z(height + 8),
                max: Vec2::new(center.x + 2, center.y + 16).with_z(height + 10),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(woodalt.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 1, center.y - 22).with_z(height + 8),
                max: Vec2::new(center.x + 1, center.y + 16).with_z(height + 10),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 2, center.y - 16).with_z(height + 9),
                max: Vec2::new(center.x + 2, center.y + 10).with_z(height + 10),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 1, center.y - 22).with_z(height + 9),
                max: Vec2::new(center.x + 1, center.y + 16).with_z(height + 10),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();

        //column
        painter
            .cylinder_with_radius(center.with_z(base), 6.0, 45.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());
        //column thick bits
        painter
            .cylinder_with_radius(center.with_z(base + 35), 7.0, 3.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());
        painter
            .cylinder_with_radius(center.with_z(base), 7.0, 1.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());

        painter
            .cylinder_with_radius(center.with_z(base), 5.0, 45.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        //lower doorway cut
        painter
            .cylinder_with_radius(
                Vec2::new(center.x - 1, center.y + 12).with_z(base - 5),
                3.5,
                7.0,
            )
            .rotate_about_min(Mat3::new(1, 0, 0, 0, 0, -1, 0, 1, 0))
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        // Base
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 11, center.y - 11).with_z(base - 16),
                max: Vec2::new(center.x + 11, center.y + 11).with_z(base),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());

        //stair cut
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 6, center.y + 9).with_z(base - 1),
                max: Vec2::new(center.x + 4, center.y + 11).with_z(base),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 5, center.y + 10).with_z(base - 2),
                max: Vec2::new(center.x + 3, center.y + 11).with_z(base - 1),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        //cone
        painter
            .cone_with_radius(center.with_z(base + 45), 8.0, 18.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        //remove 1/4 cyl
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 1, center.y + 1).with_z(height + 9),
                max: Vec2::new(center.x + 6, center.y + 6).with_z(height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x, center.y + 2).with_z(height + 9),
                max: Vec2::new(center.x + 6, center.y + 7).with_z(height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        //platform cleanup
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 2, center.y - 15).with_z(height + 8),
                max: Vec2::new(center.x + 6, center.y + 9).with_z(height + 9),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());

        //upper door
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 5, center.y - 2).with_z(height + 10),
                max: Vec2::new(center.x + 7, center.y + 2).with_z(height + 13),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 5, center.y - 1).with_z(height + 10),
                max: Vec2::new(center.x + 7, center.y + 1).with_z(height + 15),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 5, center.y - 1).with_z(height + 10),
                max: Vec2::new(center.x + 7, center.y + 1).with_z(height + 13),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        //door sprites

        let door_rot = if self.rotation == 0.0 {
            (2, 6)
        } else if self.rotation == PI / 2.0 {
            (4, 0)
        } else if self.rotation == PI {
            (6, 2) //good
        } else {
            (0, 4)
        };
        let sprite_fill = Fill::Block(Block::air(SpriteKind::Door).with_ori(door_rot.0).unwrap());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 6, center.y - 1, height + 10),
                max: Vec3::new(center.x + 7, center.y + 0, height + 11),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        let sprite_fill = Fill::Block(Block::air(SpriteKind::Door).with_ori(door_rot.1).unwrap());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 6, center.y + 0, height + 10),
                max: Vec3::new(center.x + 7, center.y + 1, height + 11),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());

        //bracing diagonal bits
        painter
            .line(
                Vec2::new(center.x + 5, center.y - 3).with_z(height),
                Vec2::new(center.x + 11, center.y - 3).with_z(height + 8),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .line(
                Vec2::new(center.x + 5, center.y + 2).with_z(height),
                Vec2::new(center.x + 11, center.y + 2).with_z(height + 8),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        //
        painter
            .line(
                Vec2::new(center.x - 11, center.y - 3).with_z(height + 8),
                Vec2::new(center.x - 6, center.y - 3).with_z(height),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .line(
                Vec2::new(center.x - 11, center.y + 2).with_z(height + 8),
                Vec2::new(center.x - 6, center.y + 2).with_z(height),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        //
        painter
            .line(
                Vec2::new(center.x - 3, center.y - 12).with_z(height + 7),
                Vec2::new(center.x - 3, center.y - 6).with_z(height),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .line(
                Vec2::new(center.x + 2, center.y - 12).with_z(height + 7),
                Vec2::new(center.x + 2, center.y - 6).with_z(height),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        //
        painter
            .line(
                Vec2::new(center.x - 3, center.y + 5).with_z(height),
                Vec2::new(center.x - 3, center.y + 9).with_z(height + 7),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .line(
                Vec2::new(center.x + 2, center.y + 5).with_z(height),
                Vec2::new(center.x + 2, center.y + 9).with_z(height + 7),
                0.8,
            )
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());

        //stairs
        painter
            .cylinder_with_radius(center.with_z(height + 8), 5.0, 1.0)
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();

        let stairs_clear1 = painter.cylinder_with_radius(center.with_z(base), 5.0, 38.0);

        painter
            .prim(Primitive::sampling(
                stairs_clear1,
                spiral_staircase(center.with_z(base + 3), 6.0, 0.5, 9.0),
            ))
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());

        //clean up interface at top
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 1, center.y + 3).with_z(height + 8),
                max: Vec2::new(center.x + 4, center.y + 5).with_z(height + 9),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(wood.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 0, center.y + 2).with_z(height + 9),
                max: Vec2::new(center.x + 6, center.y + 7).with_z(height + 10),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 1, center.y + 3).with_z(height + 9),
                max: Vec2::new(center.x + 6, center.y + 7).with_z(height + 10),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .clear();
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 0, center.y + 2).with_z(height + 9),
                max: Vec2::new(center.x + 1, center.y + 3).with_z(height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());
        //corner column
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 0, center.y + 2).with_z(height + 9),
                max: Vec2::new(center.x + 1, center.y + 3).with_z(height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(brick.clone());

        let window_rot = if self.rotation == 0.0 || self.rotation == PI {
            (2, 4)
        } else {
            (4, 2)
        };
        let sprite_fill = Fill::Block(
            Block::air(SpriteKind::Window1)
                .with_ori(window_rot.0)
                .unwrap(),
        );
        //upper window
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 6, center.y - 1).with_z(height + 12),
                max: Vec2::new(center.x - 5, center.y + 1).with_z(height + 15),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());

        //lower windows
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 6, center.y - 1).with_z(base + 19),
                max: Vec2::new(center.x - 5, center.y + 1).with_z(base + 22),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 6, center.y - 1).with_z(base + 1),
                max: Vec2::new(center.x - 5, center.y + 1).with_z(base + 4),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 5, center.y - 1).with_z(base + 4),
                max: Vec2::new(center.x + 6, center.y + 1).with_z(base + 7),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 5, center.y - 1).with_z(base + 22),
                max: Vec2::new(center.x + 6, center.y + 1).with_z(base + 25),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x + 5, center.y - 1).with_z(base + 30),
                max: Vec2::new(center.x + 6, center.y + 1).with_z(base + 33),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());

        let sprite_fill = Fill::Block(
            Block::air(SpriteKind::Window1)
                .with_ori(window_rot.1)
                .unwrap(),
        );
        //side windows
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 1, center.y + 5).with_z(base + 17),
                max: Vec2::new(center.x + 1, center.y + 6).with_z(base + 20),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - 1, center.y - 6).with_z(base + 13),
                max: Vec2::new(center.x + 1, center.y - 5).with_z(base + 16),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());

        //lights
        painter.rotated_sprite(
            Vec2::new(center.x - 3, center.y + 5).with_z(base + 8),
            SpriteKind::WallLampSmall,
            4,
        );
        painter.rotated_sprite(
            Vec2::new(center.x + 2, center.y + 5).with_z(base + 8),
            SpriteKind::WallLampSmall,
            4,
        );
        painter.rotated_sprite(
            Vec2::new(center.x - 3, center.y + 5).with_z(base + 18),
            SpriteKind::WallLampSmall,
            4,
        );
        painter.rotated_sprite(
            Vec2::new(center.x + 2, center.y + 5).with_z(base + 18),
            SpriteKind::WallLampSmall,
            4,
        );
        painter.rotated_sprite(
            Vec2::new(center.x - 3, center.y - 6).with_z(base + 8),
            SpriteKind::WallLampSmall,
            0,
        );
        painter.rotated_sprite(
            Vec2::new(center.x + 2, center.y - 6).with_z(base + 8),
            SpriteKind::WallLampSmall,
            0,
        );
        painter.rotated_sprite(
            Vec2::new(center.x - 3, center.y - 6).with_z(base + 18),
            SpriteKind::WallLampSmall,
            0,
        );
        painter.rotated_sprite(
            Vec2::new(center.x + 2, center.y - 6).with_z(base + 18),
            SpriteKind::WallLampSmall,
            0,
        );

        painter.rotated_sprite(
            Vec2::new(center.x + 5, center.y - 3).with_z(base + 13),
            SpriteKind::WallLampSmall,
            2,
        );
        painter.rotated_sprite(
            Vec2::new(center.x + 5, center.y + 2).with_z(base + 13),
            SpriteKind::WallLampSmall,
            2,
        );
        painter.rotated_sprite(
            Vec2::new(center.x + 5, center.y - 3).with_z(base + 25),
            SpriteKind::WallLampSmall,
            2,
        );
        painter.rotated_sprite(
            Vec2::new(center.x + 5, center.y + 2).with_z(base + 25),
            SpriteKind::WallLampSmall,
            2,
        );
        painter.rotated_sprite(
            Vec2::new(center.x - 6, center.y - 3).with_z(base + 13),
            SpriteKind::WallLampSmall,
            6,
        );
        painter.rotated_sprite(
            Vec2::new(center.x - 6, center.y + 2).with_z(base + 13),
            SpriteKind::WallLampSmall,
            6,
        );
        painter.rotated_sprite(
            Vec2::new(center.x - 6, center.y - 3).with_z(base + 25),
            SpriteKind::WallLampSmall,
            6,
        );
        painter.rotated_sprite(
            Vec2::new(center.x - 6, center.y + 2).with_z(base + 25),
            SpriteKind::WallLampSmall,
            6,
        );
        //upper lighting

        let sprite_fill = Fill::Block(Block::air(SpriteKind::Lantern).with_ori(2).unwrap());

        //on walkway lamps
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 2, center.y + 15, height + 7),
                max: Vec3::new(center.x - 1, center.y + 16, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 1, center.y + 15, height + 7),
                max: Vec3::new(center.x + 2, center.y + 16, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 2, center.y - 21, height + 7),
                max: Vec3::new(center.x - 1, center.y - 22, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 1, center.y - 21, height + 7),
                max: Vec3::new(center.x + 2, center.y - 22, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());

        //on platform lamps
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 11, center.y + 8, height + 7),
                max: Vec3::new(center.x - 10, center.y + 9, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 14, center.y + 8, height + 7),
                max: Vec3::new(center.x + 15, center.y + 9, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 11, center.y - 15, height + 7),
                max: Vec3::new(center.x - 10, center.y - 14, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 14, center.y - 15, height + 7),
                max: Vec3::new(center.x + 15, center.y - 14, height + 8),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        //on cone lamps
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 6, center.y + 5, height + 16),
                max: Vec3::new(center.x - 5, center.y + 6, height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 5, center.y + 5, height + 16),
                max: Vec3::new(center.x + 6, center.y + 6, height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 6, center.y - 6, height + 16),
                max: Vec3::new(center.x - 5, center.y - 5, height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x + 5, center.y - 6, height + 16),
                max: Vec3::new(center.x + 6, center.y - 5, height + 17),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());

        //interior

        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 2, center.y - 3, base + 6),
                max: Vec3::new(center.x - 1, center.y + -2, base + 7),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 2, center.y - 3, base + 15),
                max: Vec3::new(center.x - 1, center.y + -2, base + 16),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 2, center.y - 3, base + 24),
                max: Vec3::new(center.x - 1, center.y + -2, base + 25),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 2, center.y - 3, base + 33),
                max: Vec3::new(center.x - 1, center.y + -2, base + 34),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());
        painter
            .aabb(Aabb {
                min: Vec3::new(center.x - 2, center.y - 3, base + 44),
                max: Vec3::new(center.x - 1, center.y + -2, base + 45),
            })
            .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
            .fill(sprite_fill.clone());

        // crate and barrel sprites
        let mut sprite_positions = vec![];
        for a in 0..5 {
            sprite_positions.push(Vec2::new(center.x + 1 + a, center.y + 2));
        }
        for b in 0..=1 {
            sprite_positions.push(Vec2::new(center.x, center.y + 3 + b));
        }
        for sprite_pos in sprite_positions {
            let rows = (RandomField::new(0).get(sprite_pos.with_z(base)) % 3) as i32;
            for r in 0..rows {
                painter
                    .aabb(Aabb {
                        min: sprite_pos.with_z(height + 10 + r),
                        max: (sprite_pos + 1).with_z(height + 11 + r),
                    })
                    .rotate_about(Mat3::rotation_z(self.rotation).as_(), center.with_z(base))
                    .fill(Fill::Block(Block::air(
                        match (RandomField::new(0).get(sprite_pos.with_z(base + r)) % 2) as i32 {
                            0 => SpriteKind::Barrel,
                            _ => SpriteKind::CrateBlock,
                        },
                    )));
            }
        }
        // campfire
        painter.spawn(
            EntityInfo::at(self.campfire_pos.map(|e| e as f32 + 0.5))
                .into_special(SpecialEntity::Waypoint),
        );
        // docks
        for dock_pos in &self.docking_positions {
            painter
                .cylinder_with_radius(*dock_pos, 5.0, 1.0)
                .fill(wood.clone());
        }
    }
}
