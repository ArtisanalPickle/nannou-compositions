use nannou::color::conv::IntoLinSrgba;
use nannou::prelude::*;

const SIZE: f32 = 500.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<(f32, f32)>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .size(SIZE as u32, SIZE as u32)
        .build()
        .unwrap();
    Model {
        _window,
        points: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    model.points.clear();
    for p in -100..100 {
        model
            .points
            .push((((p as f32 + 5.0 * time as f32) / 2.0).sin(), p as f32))
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.time;
    let win = app.window_rect();

    draw.background().color(BLUE);

    draw.polyline()
        .color(WHITE)
        .stroke_weight(0.5)
        .points(model.points.clone());

    let mut poly_points = model.points.clone();
    poly_points.push((-SIZE, SIZE));
    poly_points.push((-SIZE, -SIZE));
    draw.polygon().color(WHITE).points(poly_points);

    for p in (-100..=100).step_by(2) {
        let right = (((p as f32 + 5.0 * time as f32) / 2.0).sin() + 1.0, p as f32);
        let left = (&right.0 * 3.0 - SIZE, &right.1 * 5.0); //(((p as f32 + 5.0 * time as f32) / 2.0).sin(), p as f32)
        draw.line()
            .weight(1.3)
            .start(pt2(right.0, right.1))
            .end(pt2(left.0, left.1))
            .color(srgba(0.0, 0.0, 1.0, 1.0));
    }

    draw.to_frame(app, &frame).unwrap();
}
