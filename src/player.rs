use og_engine::prelude::*;

pub fn calculate_player_rotation(
    initial_rot: Rotor3,
    mouse_pos: Vf2d,
    screen_size: Vf2d,
    look_radius: Vf2d,
    max_pitch: f32,
    max_yaw: f32,
    y_up_direction: f32,
) -> (Rotor3, Rotor3) {
    let screen_center = Vf2d::new(screen_size.x / 2.0, screen_size.y / 2.0);

    let snap_width_in_pixels = screen_center.x * look_radius.x;
    let snap_height_in_pixels = screen_center.y * look_radius.y;

    let mouse_offset = mouse_pos - screen_center;
    let mouse_norm = mouse_offset.norm();
    let mouse_direction: Vf2d = (mouse_norm.x, mouse_norm.y).into();

    let width_from_center = mouse_offset.x.abs();
    let height_from_center = mouse_offset.y.abs();

    let camera_yaw = num_traits::clamp(
        mouse_direction.x * (width_from_center / snap_width_in_pixels) * max_yaw,
        -max_yaw,
        max_yaw,
    );

    let camera_pitch = num_traits::clamp(
        mouse_direction.y * (height_from_center / snap_height_in_pixels) * max_pitch,
        -max_pitch,
        max_pitch,
    );

    let yaw = Rotor3::from_angle_and_axis(
        camera_yaw * std::f32::consts::PI / 180.0,
        (initial_rot.forward(), initial_rot.right()).into(),
    );

    let pitch = Rotor3::from_angle_and_axis(
        camera_pitch * std::f32::consts::PI / 180.0,
        (initial_rot.forward(), initial_rot.up()).into(),
    );

    let cam_rot = Rotor3::from_angle_and_axis(
        mouse_direction.x
            * 0.0f32.max((width_from_center - snap_width_in_pixels) / screen_center.x),
        (initial_rot.forward(), initial_rot.right()).into(),
    );

    (pitch * yaw, cam_rot)
}

pub fn update_player_camera(engine: &mut OGEngine<super::GameData>, elapsed_time: f32) {
    if engine.get_key(Key::D).held {
        let direction = engine.camera.transform.rot * Vector3::new(1.0, 0.0, 0.0);
        engine.game_data.player_velocity -= direction * elapsed_time * engine.game_data.speed;
    }

    if engine.get_key(Key::A).held {
        let direction = engine.camera.transform.rot * Vector3::new(1.0, 0.0, 0.0);
        engine.game_data.player_velocity += direction * elapsed_time * engine.game_data.speed;
    }

    if engine.get_key(Key::W).held {
        let direction = engine.camera.transform.rot * Vector3::new(0.0, 0.0, 1.0);
        engine.game_data.player_velocity += direction * elapsed_time * engine.game_data.speed;
    }

    if engine.get_key(Key::S).held {
        let direction = engine.camera.transform.rot * Vector3::new(0.0, 0.0, 1.0);
        engine.game_data.player_velocity -= direction * elapsed_time * engine.game_data.speed;
    }

    if engine.get_key(Key::K).pressed {
        engine.renderer.game_objects[0].meshes[0].calculate_normals(NormalMode::Shaded);
    }
    if engine.get_key(Key::J).pressed {
        engine.renderer.game_objects[0].meshes[0].calculate_normals(NormalMode::Flat);
    }

    engine.game_data.player += engine.game_data.player_velocity;
    engine.camera.transform.pos =
        engine.game_data.player + Vector3::new(0.0, engine.game_data.player_height, 0.0);

    engine.game_data.player_velocity *= 0.4;

    let mouse = engine.get_mouse_pos();

    if engine.get_key(Key::H).pressed {
        let mut layer = engine.get_layer_mut(engine.game_data.render_layer).unwrap();
        layer.shown = !layer.shown;
    }

    let (aim_rot, cam_rot) = calculate_player_rotation(
        engine.camera.transform.rot,
        mouse,
        (engine.window_width as f32, engine.window_height as f32).into(),
        (0.5, 0.75).into(),
        45.0,
        25.0,
        engine.get_y_up_direction(),
    );
    engine.camera.transform.rot *= cam_rot * elapsed_time * 5.0;
    //engine.camera.transform.rot = new_rot;

    draw_mouse(engine, mouse, cam_rot);
    let mut temp_camera = engine.camera;
    temp_camera.transform.rot = engine.camera.transform.rot * aim_rot;

    engine.camera.mat.view_proj = temp_camera.build_view_projection_matrix().into();
    engine.camera.mat.view_inv_proj = temp_camera.build_reverse_projection_matrix().into();

    engine.camera.mat.position = engine.camera.transform.pos.into();
}

fn draw_mouse(engine: &mut OGEngine<super::GameData>, mouse: Vf2d, cam_rot: Rotor3) {
    engine.set_draw_target(engine.game_data.ui_layer);
    {
        engine.clear(Pixel::BLANK);

        let screen: Vf2d = (engine.window_width as f32, engine.window_height as f32).into();
        let mouse_normal = (mouse - (screen / 2.0)).norm();
        if cam_rot.mag() > 0.0 {
            match mouse_normal.x.signum() > 0.0 {
                true => engine.fill_triangle(
                    mouse + Vf2d::new(25.0, 0.0),
                    mouse + Vf2d::new(-15.0, 15.0),
                    mouse + Vf2d::new(-15.0, -15.0),
                    Pixel::YELLOW,
                ),
                false => engine.fill_triangle(
                    mouse + Vf2d::new(-20.0, 0.0),
                    mouse + Vf2d::new(15.0, 15.0),
                    mouse + Vf2d::new(15.0, -15.0),
                    Pixel::YELLOW,
                ),
            }
        } else {
            engine.fill_circle(mouse, 5, Pixel::YELLOW);
        }
        engine.set_layer_update(engine.game_data.ui_layer, true);
    }
    engine.reset_draw_target();
}
