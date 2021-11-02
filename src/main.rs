#[allow(unused)]
mod player;

use olc::prelude::*;
use rust_olc_pge as olc;

fn main() {
    let game = Game::default();
    let game_data = GameData {
        speed: 10.0,
        player: (0.0, 0.0, 0.0).into(),
        player_height: 1.65,
        ..Default::default()
    };
    olc::game::construct(
        game,
        game_data,
        "Eldritch Horror",
        1280,
        720,
        1,
        1,
        false,
        false,
    );
}

#[derive(Default)]
pub struct GameData {
    ui_layer: u32,
    render_layer: u32,
    speed: f32,
    player_velocity: Vector3,
    player: Vector3,
    player_height: f32,
    listener_id: u32,
    emitter_id: u32,
}

impl OlcData for GameData {}

#[derive(Default)]
pub struct Game {}
impl Olc<GameData> for Game {
    fn on_engine_start(&self, mut engine: OLCEngine<GameData>) -> OlcFuture<OLCEngine<GameData>> {
        let fut = async {
            engine.game_data.ui_layer =
                engine.add_layer_with_info(LayerInfo::Image(layer::Image::default()));
            engine.set_layer_visible(engine.game_data.ui_layer, true);

            let render_layer = engine.add_layer(LayerType::Render);
            engine.setup_render_layer(render_layer, None);
            engine.set_layer_visible(render_layer, true);

            // let mut layer_bundle = PipelineBundle::default(&engine.renderer);
            // let shader = engine
            //     .renderer
            //     .create_shader_module(include_str!("cubify.wgsl"));
            // layer_bundle.func = LayerFunc::from(Box::new(draw_snap));
            // layer_bundle.data.update_shader(shader);
            // layer_bundle.data.rebuild_pipeline(&engine.renderer, true);
            // engine.game_data.render_layer = engine.add_layer(LayerType::Render);
            // engine.setup_render_layer(engine.game_data.render_layer, Some(layer_bundle));
            // let mut layer = engine.get_layer_mut(engine.game_data.render_layer).unwrap();
            // layer.shown = true;

            // let mut layer_bundle = PipelineBundle::default(&engine.renderer);
            // layer_bundle.func = LayerFunc::from(Box::new(draw_weird));
            // let shader = engine
            //     .renderer
            //     .create_shader_module(include_str!("dither_movement.wgsl"));
            // layer_bundle.data.update_shader(shader);
            // layer_bundle.data.rebuild_pipeline(&engine.renderer, false);
            // let weird_layer = engine.add_layer(LayerType::Render);
            // engine.setup_render_layer(weird_layer, Some(layer_bundle));
            // let mut layer = engine.get_layer_mut(weird_layer).unwrap();
            // layer.shown = true;

            // let mut layer_bundle = PipelineBundle::default(&engine.renderer);
            // layer_bundle.func = LayerFunc::from(Box::new(draw_outline));
            // let shader = engine
            //     .renderer
            //     .create_shader_module(include_str!("outline.wgsl"));
            // layer_bundle.data.update_shader(shader);
            // layer_bundle.data.rebuild_pipeline(&engine.renderer, true);
            // let outline_layer = engine.add_layer(LayerType::Render);
            // engine.setup_render_layer(outline_layer, Some(layer_bundle));
            // let mut layer = engine.get_layer_mut(outline_layer).unwrap();
            // layer.shown = true;

            let level_data = get_file_as_u8("./CellBlock.glb").await;
            let (gos, images) = gltf_ext::get_game_objects(&level_data);
            let mut current_tex_size = engine.renderer.textures.len();
            for image in images {
                engine.renderer.textures.insert(
                    engine.renderer.textures.len(),
                    texture::Texture::new_from_sprite(
                        &engine.renderer,
                        gltf_ext::image_to_sprite(&image),
                        engine.renderer.preferred_texture_format,
                    ),
                );
            }
            for mut go in gos {
                go.set_layer_mask(Mask::D3);
                // for mesh in &mut go.meshes {
                //      if let Some(tex) = mesh.textures.iter()
                // }
                engine.renderer.add_game_object(go);
            }

            engine.renderer.add_draw_data(Mask::D3);
            //engine.renderer.add_draw_data(Mask::LAYER3);
            //engine.renderer.add_draw_data(Mask::LAYER4);
            engine.camera = Camera::new();
            engine.camera.fov = 45.0;
            engine.camera.mat.view_proj = engine.camera.build_view_projection_matrix().into();
            engine
        };
        Box::pin(fut)
    }

    fn on_engine_update(
        &self,
        engine: &mut OLCEngine<GameData>,
        elapsed_time: f64,
    ) -> Result<(), &str> {
        let elapsed_time = elapsed_time as f32;
        player::update_player_camera(engine, elapsed_time);

        //let mut dirty = false;
        // if dirty {
        //engine.renderer.update_layer_draw_data(Mask::LAYER4);
        // }

        if engine.get_key(Key::Escape).pressed {
            Err("Ended")
        } else {
            Ok(())
        }
    }

    fn on_engine_destroy(&self, _engine: &mut OLCEngine<GameData>) -> Result<(), &str> {
        Ok(())
    }
}

pub fn draw_outline(
    layer: &LayerDesc<GameData>,
    renderer: &Renderer,
    _game_data: &mut GameData,
    encoder: &mut wgpu::CommandEncoder,
) {
    if let LayerInfo::Render(render_info) = &layer.layer_info {
        if let Some(pipeline_data) = &render_info.pipeline_bundle {
            renderer.draw_mask(
                &renderer.camera,
                Mask::LAYER4,
                &renderer.frame_texture_backbuffer,
                Some(wgpu::Color::TRANSPARENT),
                true,
                None, //Some(&render_info.pipeline_bundle.as_ref().unwrap().data.pipeline),
            );

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Layer Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &renderer.frame_texture.texture_bundle.as_ref().unwrap().view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &renderer.depth_texture.texture_bundle.as_ref().unwrap().view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });
            render_pass.set_pipeline(&pipeline_data.data.pipeline);
            for (i, bg) in pipeline_data.data.bind_groups.iter().enumerate() {
                render_pass.set_bind_group(i as u32, bg, &[]);
            }
            render_pass.set_vertex_buffer(0, renderer.decal_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }
    }
}

pub fn draw_weird(
    layer: &LayerDesc<GameData>,
    renderer: &Renderer,
    _game_data: &mut GameData,
    encoder: &mut wgpu::CommandEncoder,
) {
    if let LayerInfo::Render(render_info) = &layer.layer_info {
        if let Some(pipeline_data) = &render_info.pipeline_bundle {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Layer Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &renderer.frame_texture.texture_bundle.as_ref().unwrap().view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&pipeline_data.data.pipeline);
            for (i, bg) in pipeline_data.data.bind_groups.iter().enumerate() {
                render_pass.set_bind_group(i as u32, bg, &[]);
            }
            render_pass.set_vertex_buffer(0, renderer.decal_buffer.slice(..));
            render_pass.draw(0..6, 0..1);
        }
    }
}

pub fn draw_snap(
    layer: &LayerDesc<GameData>,
    renderer: &Renderer,
    _game_data: &mut GameData,
    _encoder: &mut wgpu::CommandEncoder,
) {
    renderer.draw_mask(
        &renderer.camera,
        Mask::D3 & Mask::LAYER3,
        &renderer.frame_texture,
        Some(wgpu::Color::TRANSPARENT),
        true,
        None,
    );
    // if let LayerInfo::Render(render_info) = &layer.layer_info {
    //     renderer.draw_mask(
    //         &renderer.camera,
    //         Mask::LAYER3,
    //         &renderer.frame_texture_backbuffer,
    //         Some(wgpu::Color::TRANSPARENT),
    //         true,
    //         Some(&render_info.pipeline_bundle.as_ref().unwrap().data.pipeline),
    //     );
    // }
}

pub fn create_dither_pipeline(_game_data: &GameData) -> PipelineBundle<GameData> {
    todo!()
}

// #[test]
// fn check_bytemuck(){
//     let v: Vec<Vertex> = vec![[0.0, 1.0, 5.0].into(); 6];
//     let t: Vec<Triangle> = vec![[Vector3::new(0.0, 1.0, 5.0); 3].into(),
//                                 [Vector3::new(0.0, 1.0, 5.0); 3].into()];
//     let t_v: &[Vertex] = bytemuck::cast_slice(t.as_slice());
//     assert_eq!(
//         bytemuck::cast_slice::<Vertex, u8>(v.as_slice()),
//         bytemuck::cast_slice::<Triangle, u8>(t.as_slice())
//     )
// }
