// use nannou::prelude::*;
// use std::ops::Range;

// const NUM_POINTS: i32 = 100;
// const SIZE: f32 = 700.0;

// fn main() {
//     nannou::app(model).run();
// }

// struct Model {
//     _window: window::Id,
//     texture: wgpu::Texture,
// }

// fn model(app: &App) -> Model {
//     let _window = app
//         .new_window()
//         .size(SIZE as u32, SIZE as u32)
//         .view(view)
//         .build()
//         .unwrap();
//     let img_path = app
//         .assets_path()
//         .unwrap()
//         .join("images")
//         .join("nature")
//         .join("nature_1.jpg");
//     let texture = wgpu::Texture::from_path(app, img_path).unwrap();
//     Model { _window, texture }
// }

// // fn update(_app: &App, _model: &mut Model, _update: Update) {}

// fn view(app: &App, model: &Model, frame: Frame) {
//     let draw = app.draw();

//     if app.elapsed_frames() == 1 {
//         draw.background().color(BLACK);
//     }

//     draw.texture(&model.texture);

//     draw.rect().w_h(SIZE, SIZE).color(srgba(0.0, 0.0, 0.0, 0.1));

//     let time = app.time;
//     let freq = 10.0;
//     let radius = (time.sin()) * 50.0 + 200.0;

//     // let mut r = Range{ start: 0.0, end: 2.0 * PI};
//     let mut points: Vec<(f32, f32)> = (0..NUM_POINTS)
//         .map(|i| 2.0 * PI * i as f32 / NUM_POINTS as f32)
//         .map(|p| {
//             (
//                 ((p as f32).sin() * radius) + (freq * time * p).tan(),
//                 ((p as f32).cos() * radius) + (freq * time * p).tan(),
//             )
//         })
//         .collect();
//     points.push((0.0, radius)); // finish the circle

//     draw.polygon().color(WHITE).points(points);

//     draw.to_frame(app, &frame).unwrap();
// }

//! A simple as possible example demonstrating how to use the `draw` API to display a texture.

// use nannou::prelude::*;

// fn main() {
//     nannou::app(model).run();
// }

// struct Model {
//     texture: wgpu::Texture,
// }

// fn model(app: &App) -> Model {
//     // Create a new window! Store the ID so we can refer to it later.
//     app.new_window().size(512, 512).view(view).build().unwrap();
//     // Load the image from disk and upload it to a GPU texture.
//     let assets = app.assets_path().unwrap();
//     let img_path = assets.join("images").join("nature").join("nature_1.jpg");
//     let texture = wgpu::Texture::from_path(app, img_path).unwrap();
//     Model { texture }
// }

// // Draw the state of your `Model` into the given `Frame` here.
// fn view(app: &App, model: &Model, frame: Frame) {
//     frame.clear(BLACK);
//     let win = app.main_window();
//     let win_r = win.rect();

//     // Let's choose the address mode based on the mouse position.
//     let address_mode = match map_range(app.mouse.y, win_r.top(), win_r.bottom(), 0.0, 3.0) as i8 {
//         0 => wgpu::AddressMode::ClampToEdge,
//         1 => wgpu::AddressMode::Repeat,
//         _ => wgpu::AddressMode::MirrorRepeat,
//     };

//     // Create a sampler with the chosen address mode.
//     let sampler = wgpu::SamplerBuilder::new()
//         .address_mode(address_mode)
//         .into_descriptor();

//     // At any point during drawing, we can create a new `draw` context that will let us use a
//     // different sampler.
//     let draw = app.draw();
//     let draw = draw.sampler(sampler);

//     // Change the texture coordinates to sample outside the texture. This will demonstrate how the
//     // sampler behaves when sampling beyond the bounds of the texture in each of the different
//     // address modes. By default, the bounds of the texture coordinates are 0.0 to 1.0. We will
//     // triple the size.
//     let area = geom::Rect::from_x_y_w_h(0.5, 0.5, app.time.sin() * 10.0, app.time.sin() * 10.0);

//     draw.texture(&model.texture).area(area);

//     // Draw the current address mode in the bottom left corner.
//     let text = format!("Address mode: {:?}", address_mode);
//     draw.text(&text)
//         .wh(win_r.wh() * 0.95)
//         .left_justify()
//         .align_text_bottom();

//     draw.to_frame(app, &frame).unwrap();
// }
use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: window::Id,
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().size(512, 512).view(view).build().unwrap();

    // Load the image from disk and upload it to a GPU texture.
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("nature").join("nature_1.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model { window_id, texture }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
    let window = app.window(model.window_id).unwrap();
    let win_rect = window.rect();
    let draw = app.draw();

    // Generate the triangulated points for a cuboid to use for out mesh.
    let centre = pt3(0.0, 0.0, 0.0);
    let size = vec3(1.0, 1.0, 1.0);
    let cuboid = geom::Cuboid::from_xyz_whd(centre, size);
    let points = cuboid
        .triangles_iter()
        .flat_map(geom::Tri::vertices)
        .map(|point| {
            // Tex coords should be in range (0.0, 0.0) to (1.0, 1.0);
            // This will have the logo show on the front and back faces.
            let tex_coords = [point.x + 0.5, 1.0 - (point.y + 0.5)];
            (point, tex_coords)
        });

    // Scale the points up to half the window size.
    let cube_side = win_rect.w().min(win_rect.h()) * 0.5;
    draw.scale(cube_side)
        .mesh()
        .points_textured(&model.texture, points)
        .z_radians(app.time * 0.33)
        .x_radians(app.time * 0.166 + -app.mouse.y / 100.0)
        .y_radians(app.time * 0.25 + app.mouse.x / 100.0);

    // Draw to the frame!
    draw.to_frame(app, &frame).unwrap();
}
