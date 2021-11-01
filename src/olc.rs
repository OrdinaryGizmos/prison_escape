#![allow(non_snake_case)]
#![allow(deprecated)]
#![allow(clippy::too_many_arguments)]
//#![feature(nll)]

// pub mod app;
pub mod camera;
// pub mod debug_gui;
pub mod decal;
pub mod engine;
pub mod game;
pub mod game_object;
pub mod geometry;
pub mod gltf_ext;
pub mod layer;
pub mod math_3d;
pub mod math_4d;
pub mod pixel;
pub mod platform;
pub mod renderer;
pub mod sprite;
pub mod texture;
pub mod transform;
pub mod util;
use engine::OLCEngine;

#[derive(Debug)]
pub enum Rcode {
    Fail,
    Ok,
    NoFile,
}

pub type OlcFuture<T> = std::pin::Pin<Box<dyn std::future::Future<Output = T>>>;

pub trait Olc<D: 'static + OlcData> {
    fn on_engine_start(&self, engine: OLCEngine<D>) -> Result<OlcFuture<OLCEngine<D>>, &str>;

    fn on_engine_update(&self, engine: &mut OLCEngine<D>, elapsedTime: f64) -> Result<(), &str>;

    fn on_engine_destroy(&self, engine: &mut OLCEngine<D>) -> Result<(), &str>;
}

pub trait OlcData {}

pub mod prelude {
    pub use crate::olc::{
        camera, camera::*, decal, decal::*, engine, engine::*, game, game::*, game_object,
        game_object::*, geometry, geometry::*, gltf_ext, gltf_ext::*, layer, layer::*, math_3d, math_3d::*, math_4d,
        math_4d::*, pixel, pixel::*, platform, platform::*, renderer, renderer::*, sprite,
        sprite::*, texture, texture::*, transform, transform::*, util, util::*, Olc, OlcData,
        OlcFuture, Rcode,
    };
}
