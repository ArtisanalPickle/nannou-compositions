use nannou::prelude::*;

const SIZE: i32 = 500;

fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

struct Model {
    _window: window::Id,
    points: Vec<Point>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .build()
        .unwrap();

    let points = vec![Point::new(0.0, 0.0, 0.0)];

    Model { _window, points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let secs = app.elapsed_frames() as f32 / 60.0;

    for p in model.points.iter_mut() {
        // p.x = secs.sin() * 100.0;
        p.y = secs.sin() * 100.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .w_h(SIZE as f32, SIZE as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.1));
    for p in model.points.iter() {
        draw.ellipse().x_y(p.x, p.y).color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}
