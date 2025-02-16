use super::*;
use crate::{
    Land,
    site2::gen::place_circular,
    util::{CARDINALS, RandomField, Sampler},
};
use common::{
    comp::Content,
    generation::SpecialEntity,
    terrain::{BlockKind, SpriteCfg, SpriteKind},
};
use rand::prelude::*;
use std::sync::Arc;
use vek::*;

/// Represents house data generated by the `generate()` method
pub struct CoastalAirshipDock {
    /// Tile position of the door tile
    pub door_tile: Vec2<i32>,
    /// Approximate altitude of the door tile
    pub(crate) alt: i32,
    base: i32,
    pub center: Vec2<i32>,
    size: i32,
    bldg_height: i32,
    diameter: i32,
    pub docking_positions: Vec<Vec3<i32>>,
}

impl CoastalAirshipDock {
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
        let diameter = (bounds.max.x - bounds.min.x).min(bounds.max.y - bounds.min.y);
        let alt = land.get_alt_approx(site.tile_center_wpos(door_tile + door_dir)) as i32 + 2;
        let size = (diameter / 3) + 2;
        let bldg_height = 12;
        let base = alt + 1;
        let center = bounds.center();
        let mut docking_positions = vec![];
        let top_floor = base + (bldg_height * 6) - 3;
        for dir in CARDINALS {
            let docking_pos = center + dir * size;
            docking_positions.push(docking_pos.with_z(top_floor));
        }
        Self {
            door_tile: door_tile_pos,
            alt,
            base,
            center,
            size,
            bldg_height,
            diameter,
            docking_positions,
        }
    }
}

impl Structure for CoastalAirshipDock {
    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"render_coastal_airship_dock\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "render_coastal_airship_dock")]
    fn render_inner(&self, _site: &Site, _land: &Land, painter: &Painter) {
        let base = self.base;
        let center = self.center;
        let white = Fill::Sampling(Arc::new(|center| {
            Some(match (RandomField::new(0).get(center)) % 37 {
                0..=8 => Block::new(BlockKind::Rock, Rgb::new(251, 251, 227)),
                9..=17 => Block::new(BlockKind::Rock, Rgb::new(245, 245, 229)),
                18..=26 => Block::new(BlockKind::Rock, Rgb::new(250, 243, 221)),
                27..=35 => Block::new(BlockKind::Rock, Rgb::new(240, 240, 230)),
                _ => Block::new(BlockKind::Rock, Rgb::new(255, 244, 193)),
            })
        }));
        let blue_broken = Fill::Sampling(Arc::new(|center| {
            Some(match (RandomField::new(0).get(center)) % 20 {
                0 => Block::new(BlockKind::Rock, Rgb::new(30, 187, 235)),
                _ => Block::new(BlockKind::Rock, Rgb::new(11, 146, 187)),
            })
        }));

        let length = self.diameter / 2;
        let width = (self.diameter / 2) - 1;
        let height = 15;
        // fence, blue gates
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - length - 6, center.y - width - 6).with_z(base - 2),
                max: Vec2::new(center.x + length + 7, center.y + width + 7).with_z(base - 1),
            })
            .fill(blue_broken.clone());

        for dir in CARDINALS {
            let frame_pos = Vec2::new(
                center.x + dir.x * (length + 5),
                center.y + dir.y * (width + 5),
            );
            painter
                .line(center.with_z(base - 1), frame_pos.with_z(base - 1), 3.0)
                .fill(blue_broken.clone());
        }
        // foundation
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - length - 6, center.y - width - 6).with_z(base - height),
                max: Vec2::new(center.x + length + 7, center.y + width + 7).with_z(base - 2),
            })
            .fill(white.clone());
        for f in 0..8 {
            painter
                .aabb(Aabb {
                    min: Vec2::new(center.x - length - 7 - f, center.y - width - 7 - f)
                        .with_z(base - 3 - f),
                    max: Vec2::new(center.x + length + 8 + f, center.y + width + 8 + f)
                        .with_z(base - 2 - f),
                })
                .fill(white.clone());
        }
        // clear yard
        painter
            .aabb(Aabb {
                min: Vec2::new(center.x - length - 5, center.y - width - 5).with_z(base - 2),
                max: Vec2::new(center.x + length + 6, center.y + width + 6).with_z(base + height),
            })
            .clear();
        // clear entries
        for dir in CARDINALS {
            let clear_pos = Vec2::new(
                center.x + dir.x * (length + 7),
                center.y + dir.y * (width + 7),
            );
            painter
                .line(center.with_z(base - 1), clear_pos.with_z(base - 1), 2.0)
                .clear();
        }

        // rooms
        let size = self.size;
        let room_offset = size / 6;
        let bldg_height = self.bldg_height;
        for r in 0..=4 {
            let bldg_size = size - (room_offset * r);
            let bldg_base = base + ((bldg_height + 2) * r);
            if r == 4 {
                painter
                    .cylinder(Aabb {
                        min: (center - bldg_size - 2).with_z(bldg_base + bldg_height - 1),
                        max: (center + bldg_size + 2).with_z(bldg_base + bldg_height),
                    })
                    .fill(white.clone());
                painter
                    .cylinder(Aabb {
                        min: (center - bldg_size - 2).with_z(bldg_base + bldg_height),
                        max: (center + bldg_size + 2).with_z(bldg_base + bldg_height + 1),
                    })
                    .fill(blue_broken.clone());
                painter
                    .cylinder(Aabb {
                        min: (center - bldg_size - 1).with_z(bldg_base + bldg_height),
                        max: (center + bldg_size + 1).with_z(bldg_base + bldg_height + 1),
                    })
                    .clear();

                let cargo_pos = Vec2::new(center.x, center.y + 5);
                for dir in CARDINALS {
                    let sprite_pos = cargo_pos + dir;
                    let rows = 1 + (RandomField::new(0).get(sprite_pos.with_z(base)) % 3) as i32;
                    for r in 0..rows {
                        painter
                            .aabb(Aabb {
                                min: (sprite_pos).with_z(bldg_base + bldg_height + r),
                                max: (sprite_pos + 1).with_z(bldg_base + bldg_height + 1 + r),
                            })
                            .fill(Fill::Block(Block::air(
                                match (RandomField::new(0).get(sprite_pos.with_z(base + r)) % 2)
                                    as i32
                                {
                                    0 => SpriteKind::Barrel,
                                    _ => SpriteKind::CrateBlock,
                                },
                            )));
                        if r > 1 {
                            painter.owned_resource_sprite(
                                sprite_pos.with_z(bldg_base + bldg_height + 1 + r),
                                SpriteKind::Crate,
                                0,
                            );
                        }
                    }

                    // docks
                    let gangway_pos = center + dir * (size / 2);
                    let dock_pos = center + dir * (size - 3);
                    painter
                        .aabb(Aabb {
                            min: (gangway_pos - 3).with_z(bldg_base + bldg_height - 1),
                            max: (gangway_pos + 3).with_z(bldg_base + bldg_height),
                        })
                        .fill(white.clone());
                    painter
                        .cylinder(Aabb {
                            min: (dock_pos - 4).with_z(bldg_base + bldg_height),
                            max: (dock_pos + 4).with_z(bldg_base + bldg_height + 1),
                        })
                        .fill(blue_broken.clone());
                    painter
                        .cylinder(Aabb {
                            min: (dock_pos - 3).with_z(bldg_base + bldg_height - 1),
                            max: (dock_pos + 3).with_z(bldg_base + bldg_height + 1),
                        })
                        .fill(white.clone());
                }
                // campfire
                let campfire_pos = center.with_z(bldg_base + bldg_height);
                painter.spawn(
                    EntityInfo::at(campfire_pos.map(|e| e as f32 + 0.5))
                        .into_special(SpecialEntity::Waypoint),
                );
            }
            painter
                .cylinder(Aabb {
                    min: (center - bldg_size).with_z(bldg_base - 2),
                    max: (center + bldg_size).with_z(bldg_base + bldg_height),
                })
                .fill(white.clone());
        }
        for r in 0..=4 {
            let bldg_size = size - (room_offset * r);
            let bldg_base = base + ((bldg_height + 2) * r);

            let step_positions = place_circular(center, (bldg_size - 1) as f32, 14);
            for (s, step_pos) in step_positions.enumerate() {
                let step_size = (size / 3) - r;

                painter
                    .cylinder(Aabb {
                        min: (step_pos - step_size).with_z(bldg_base - 2 + s as i32),
                        max: (step_pos + step_size).with_z(bldg_base + 4 + s as i32),
                    })
                    .clear();
                painter
                    .cylinder(Aabb {
                        min: (step_pos - step_size).with_z(bldg_base - 3 + s as i32),
                        max: (step_pos + step_size).with_z(bldg_base - 2 + s as i32),
                    })
                    .fill(blue_broken.clone());
                painter
                    .cylinder(Aabb {
                        min: (step_pos - step_size + 1).with_z(bldg_base - 4 + s as i32),
                        max: (step_pos + step_size - 1).with_z(bldg_base - 2 + s as i32),
                    })
                    .fill(white.clone());
            }
            let lamp_positions = place_circular(center, (bldg_size + 1) as f32, 14);
            for (l, lamp_pos) in lamp_positions.enumerate() {
                if (RandomField::new(0).get(lamp_pos.with_z(base)) % 4) < 1 {
                    painter
                        .aabb(Aabb {
                            min: (lamp_pos - 1).with_z(bldg_base - 3 + l as i32),
                            max: (lamp_pos + 1).with_z(bldg_base - 2 + l as i32),
                        })
                        .fill(blue_broken.clone());

                    painter.sprite(
                        lamp_pos.with_z(bldg_base - 2 + l as i32),
                        SpriteKind::FireBowlGround,
                    );
                }
            }
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
    }
}
