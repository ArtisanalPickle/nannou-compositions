use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

const SIZE: f32 = 500.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .build()
        .unwrap();
    let noise = Perlin::new();

    Model { _window, noise }
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let time = app.elapsed_frames();
    let noise = model.noise;

    draw.background().color(BLACK);

    let colored_tris = (-200..=200)
        .step_by(9)
        .flat_map(|i| {
            (-200..=200).step_by(9).flat_map(move |j| {
                let l_x = i as f32;
                let r_x = l_x + 7.0;
                let bottom_y = j as f32;
                let top_y = bottom_y + 7.0;

                let top_left = pt2(l_x, top_y);
                let top_right = pt2(r_x, top_y);
                let bottom_right = pt2(r_x, bottom_y);
                let bottom_left = pt2(l_x, bottom_y);
                geom::Quad([top_left, top_right, bottom_right, bottom_left]).triangles_iter()
            })
        })
        .map(|tri| {
            tri.map_vertices(|v| {
                let x_val = (v.x as f64 * 0.01) + (0.01 * time as f64);
                let y_val = (v.y as f64 * 0.001).sin() + (0.01 * time as f64);

                let color = srgba(
                    noise.get([(time as f64 + x_val * 0.1), (time as f64 + y_val * 0.1)]) * 0.3,
                    noise.get([x_val, y_val]) / 3.0,
                    noise.get([x_val, y_val]),
                    1.0,
                );
                // let color = srgba(0.0, 0.0, 1.0, 1.0);
                (v, color)
            })
        });

    draw.mesh().tris_colored(colored_tris);
    // draw.blend(blend_descriptor: wgpu::BlendDescriptor)
    draw.to_frame(app, &frame).unwrap();
}
