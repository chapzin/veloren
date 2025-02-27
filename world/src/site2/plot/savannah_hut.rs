use super::*;
use crate::{
    Land,
    util::{CARDINALS, DIAGONALS, RandomField, Sampler},
};
use common::terrain::{BlockKind, SpriteKind, sprite::Owned};
use rand::prelude::*;
use std::{f32::consts::TAU, sync::Arc};
use vek::*;

/// Represents house data generated by the `generate()` method
pub struct SavannahHut {
    /// Tile position of the door tile
    pub door_tile: Vec2<i32>,
    /// Axis aligned bounding region for the house
    bounds: Aabr<i32>,
    /// Approximate altitude of the door tile
    pub(crate) alt: i32,
}

impl SavannahHut {
    pub fn generate(
        land: &Land,
        _rng: &mut impl Rng,
        site: &Site,
        door_tile: Vec2<i32>,
        door_dir: Vec2<i32>,
        tile_aabr: Aabr<i32>,
        alt: Option<i32>,
    ) -> Self {
        let door_tile_pos = site.tile_center_wpos(door_tile);
        let bounds = Aabr {
            min: site.tile_wpos(tile_aabr.min),
            max: site.tile_wpos(tile_aabr.max),
        };
        Self {
            bounds,
            door_tile: door_tile_pos,
            alt: alt.unwrap_or_else(|| {
                land.get_alt_approx(site.tile_center_wpos(door_tile + door_dir)) as i32
            }) + 2,
        }
    }
}

impl Structure for SavannahHut {
    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"render_savannahhut\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "render_savannahhut")]
    fn render_inner(&self, _site: &Site, _land: &Land, painter: &Painter) {
        let base = self.alt + 1;
        let center = self.bounds.center();
        let sprite_fill = Fill::Sampling(Arc::new(|wpos| {
            Some(match (RandomField::new(0).get(wpos)) % 25 {
                0 => Block::air(SpriteKind::Bowl).with_attr(Owned(true)).unwrap(),
                1 => Block::air(SpriteKind::VialEmpty)
                    .with_attr(Owned(true))
                    .unwrap(),
                2 => Block::air(SpriteKind::Lantern),
                3 => Block::air(SpriteKind::JugArabic),
                4 => Block::air(SpriteKind::Crate)
                    .with_attr(Owned(true))
                    .unwrap(),
                _ => Block::new(BlockKind::Air, Rgb::new(0, 0, 0)),
            })
        }));
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
        let length = (10 + RandomField::new(0).get(center.with_z(base)) % 6) as i32;
        let height = 2 * length / 3;
        let storeys = (1 + RandomField::new(0).get(center.with_z(base)) % 2) as i32;
        let radius = length + (length / 3);
        let reed_var = (1 + RandomField::new(0).get(center.with_z(base)) % 4) as f32;
        let reed_parts = 36_f32 + reed_var;
        let phi = TAU / reed_parts;
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
                min: (center - length + 4).with_z(base),
                max: (center + 1 + length - 4).with_z(base + 1),
            })
            .fill(sprite_fill);

        painter
            .cylinder(Aabb {
                min: (center - length + 4).with_z(base + (storeys * height) - 1),
                max: (center + 1 + length - 4).with_z(base + (storeys * height) + 1),
            })
            .fill(wood_dark);

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
            .fill(color);
        painter
            .cylinder(Aabb {
                min: (center - (length / 2) + 1).with_z(base - 3),
                max: (center + (length / 2) - 1).with_z(base - 2),
            })
            .fill(clay);

        // furniture
        let mut sprites = vec![
            SpriteKind::CushionArabic,
            SpriteKind::TableArabicSmall,
            SpriteKind::DecorSetArabic,
            SpriteKind::Bowl,
            SpriteKind::VialEmpty,
            SpriteKind::JugArabic,
            SpriteKind::JugAndBowlArabic,
            SpriteKind::SepareArabic,
        ];

        let rows = if length > 12 { 2 } else { 1 };
        'outer: for d in 0..rows {
            for dir in DIAGONALS {
                if sprites.is_empty() {
                    break 'outer;
                }
                let position = center + dir * (length - (9 + (d * 3)));
                let sprite = sprites.swap_remove(
                    RandomField::new(0).get(position.with_z(base)) as usize % sprites.len(),
                );
                painter.owned_resource_sprite(position.with_z(base - 2), sprite, 0);
            }
        }

        // draws a random index
        let random_index = (RandomField::new(0).get(center.with_z(base)) % 4) as usize;
        // add bed at random diagonal
        let random_diag = DIAGONALS.get(random_index).unwrap();
        let bed_pos = center + random_diag * (length - 9);
        painter.sprite(bed_pos.with_z(base - 2), SpriteKind::BedWoodWoodlandHead);

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
}
