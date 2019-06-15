use crate::{all::ForestKind, util::Sampler, World, CONFIG};
use common::{
    terrain::{Block, TerrainChunkSize},
    vol::{VolSize, Vox},
};
use noise::NoiseFn;
use std::ops::{Add, Div, Mul, Neg, Sub};
use vek::*;

pub struct ColumnGen<'a> {
    world: &'a World,
}

impl<'a> ColumnGen<'a> {
    pub fn new(world: &'a World) -> Self {
        Self { world }
    }
}

impl<'a> Sampler for ColumnGen<'a> {
    type Index = Vec2<i32>;
    type Sample = Option<ColumnSample>;

    fn get(&self, wpos: Vec2<i32>) -> Option<ColumnSample> {
        let wposf = wpos.map(|e| e as f64);
        let chunk_pos = wpos.map2(Vec2::from(TerrainChunkSize::SIZE), |e, sz: u32| {
            e as u32 / sz
        });

        let sim = self.world.sim();

        let alt_base = sim.get_interpolated(wpos, |chunk| chunk.alt_base)?;
        let chaos = sim.get_interpolated(wpos, |chunk| chunk.chaos)?;
        let temp = sim.get_interpolated(wpos, |chunk| chunk.temp)?;
        let rockiness = sim.get_interpolated(wpos, |chunk| chunk.rockiness)?;
        let cliffiness = sim.get_interpolated(wpos, |chunk| chunk.cliffiness)?;
        let tree_density = sim.get_interpolated(wpos, |chunk| chunk.tree_density)?;

        let forest_kind = sim.get(chunk_pos)?.forest_kind;

        let alt = sim.get_interpolated(wpos, |chunk| chunk.alt)?
            + sim.gen_ctx.small_nz.get((wposf.div(256.0)).into_array()) as f32
                * chaos.max(0.2)
                * 64.0;

        let rock = (sim.gen_ctx.small_nz.get(
            Vec3::new(wposf.x, wposf.y, alt as f64)
                .div(100.0)
                .into_array(),
        ) as f32)
            .mul(rockiness)
            .sub(0.4)
            .max(0.0)
            .mul(8.0);

        let wposf3d = Vec3::new(wposf.x, wposf.y, alt as f64);

        let marble = (0.0
            + (sim.gen_ctx.hill_nz.get((wposf3d.div(48.0)).into_array()) as f32).mul(0.75)
            + (sim.gen_ctx.hill_nz.get((wposf3d.div(3.0)).into_array()) as f32).mul(0.25))
        .add(1.0)
        .mul(0.5);

        // Colours
        let cold_grass = Rgb::new(0.0, 0.3, 0.1);
        let warm_grass = Rgb::new(0.35, 1.0, 0.05);
        let cold_stone = Rgb::new(0.55, 0.7, 0.75);
        let warm_stone = Rgb::new(0.65, 0.65, 0.35);
        let beach_sand = Rgb::new(0.93, 0.84, 0.4);
        let desert_sand = Rgb::new(0.98, 0.8, 0.15);
        let snow = Rgb::broadcast(1.0);

        let grass = Rgb::lerp(cold_grass, warm_grass, marble);
        let sand = Rgb::lerp(beach_sand, desert_sand, marble);
        let cliff = Rgb::lerp(cold_stone, warm_stone, marble);

        let ground = Rgb::lerp(
            Rgb::lerp(
                snow,
                grass,
                temp.sub(CONFIG.snow_temp)
                    .sub((marble - 0.5) * 0.05)
                    .mul(256.0),
            ),
            sand,
            temp.sub(CONFIG.desert_temp).mul(32.0),
        );

        // Caves
        let cave_at = |wposf: Vec2<f64>| {
            (sim.gen_ctx.cave_0_nz.get(
                Vec3::new(wposf.x, wposf.y, alt as f64 * 8.0)
                    .div(800.0)
                    .into_array(),
            ) as f32)
                .powf(2.0)
                .neg()
                .add(1.0)
                .mul((1.15 - chaos).min(1.0))
        };
        let cave_xy = cave_at(wposf);
        let cave_alt = alt - 32.0
            + (sim
                .gen_ctx
                .cave_1_nz
                .get(Vec2::new(wposf.x, wposf.y).div(48.0).into_array()) as f32)
                * 8.0
            + (sim
                .gen_ctx
                .cave_1_nz
                .get(Vec2::new(wposf.x, wposf.y).div(300.0).into_array()) as f32)
                .add(1.0)
                .mul(0.5)
                .powf(8.0)
                .mul(256.0);

        Some(ColumnSample {
            alt,
            chaos,
            surface_color: Rgb::lerp(
                sand,
                // Land
                Rgb::lerp(
                    ground,
                    // Mountain
                    Rgb::lerp(
                        cliff,
                        snow,
                        (alt - CONFIG.sea_level
                            - 0.35 * CONFIG.mountain_scale
                            - alt_base
                            - temp * 96.0
                            - marble * 24.0)
                            / 12.0,
                    ),
                    (alt - CONFIG.sea_level - 0.2 * CONFIG.mountain_scale) / 180.0,
                ),
                // Beach
                (alt - CONFIG.sea_level - 2.0) / 5.0,
            ),
            tree_density,
            forest_kind,
            close_trees: sim.gen_ctx.tree_gen.get(wpos),
            cave_xy,
            cave_alt,
            rock,
            cliff: cliffiness,
            temp,
        })
    }
}

#[derive(Clone)]
pub struct ColumnSample {
    pub alt: f32,
    pub chaos: f32,
    pub surface_color: Rgb<f32>,
    pub tree_density: f32,
    pub forest_kind: ForestKind,
    pub close_trees: [(Vec2<i32>, u32); 9],
    pub cave_xy: f32,
    pub cave_alt: f32,
    pub rock: f32,
    pub cliff: f32,
    pub temp: f32,
}
