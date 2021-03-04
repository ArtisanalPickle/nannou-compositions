use nannou::prelude::*;

const SIZE: i32 = 500;

fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    x: f32,
    y: f32,
    z: f32,
    size: f32,
}

impl Point {
    fn new(x: f32, y: f32, z: f32, size: f32) -> Point {
        Point { x, y, z, size }
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

    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new(0.0, 0.0, 0.0, 100.0));

    Model { _window, points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let secs = app.elapsed_frames() as f32 / 60.0;

    if model.points.len() < 100 {
        let p = model.points.last().unwrap();
        model.points.push(Point::new(p.x, p.y, p.z, p.size));
    }

    let mut i = 0;
    for p in model.points.iter_mut() {
        p.y = (secs - (i as f32 * 0.1)).sin() * 100.0;
        p.size = (secs - (i as f32 * 0.1)).sin() * 100.0; // i as f32;
        i += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // if app.elapsed_frames() == 1 {
    draw.background().color(BLACK);
    // }
    // draw.rect()
    //     .w_h(SIZE as f32, SIZE as f32)
    //     .color(srgba(0.0, 0.0, 0.0, 0.1));
    for p in model.points.iter() {
        draw.ellipse()
            .w_h(p.size, p.size)
            .z(p.size)
            .x_y(p.size, p.size)
            .color(srgba(p.size / 100.0, 1.0, p.size / 100.0, p.size / 100.0));
    }
    draw.to_frame(app, &frame).unwrap();
}
