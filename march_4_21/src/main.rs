use nannou::prelude::*;
use std::ops::Range;

const NUM_POINTS: i32 = 100;
const SIZE: f32 = 700.0;

fn main() {
    nannou::app(model).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .build()
        .unwrap();
    Model { _window }
}

// fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.rect().w_h(SIZE, SIZE).color(srgba(0.0, 0.0, 0.0, 0.1));

    let time = app.time;
    let freq = 10.0;
    let radius = (time.sin()) * 50.0 + 200.0;

    // let mut r = Range{ start: 0.0, end: 2.0 * PI};
    let mut points: Vec<(f32, f32)> = (0..NUM_POINTS)
        .map(|i| 2.0 * PI * i as f32 / NUM_POINTS as f32)
        .map(|p| {
            (
                ((p as f32).sin() * radius) + (freq * time * p).tan(),
                ((p as f32).cos() * radius) + (freq * time * p).tan(),
            )
        })
        .collect();
    points.push((0.0, radius)); // finish the circle

    draw.polyline().color(WHITE).weight(0.5).points(points);

    draw.to_frame(app, &frame).unwrap();
}
