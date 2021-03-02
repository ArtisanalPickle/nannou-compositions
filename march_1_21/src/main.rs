use nannou::prelude::*;

const SIZE: f32 = 500.0;
const POINT_SIZE: f32 = 10.0;
const MARGIN: f32 = 100.0;
const NUM_COL: i32 = 31; //(SIZE - (2.0 * MARGIN) / POINT_SIZE) as i32 + 1;

fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    pos: Vector3<f32>,
    rot: Vector3<f32>,
}

impl Point {
    fn new(pos: Vector3<f32>, rot: Vector3<f32>) -> Point {
        Point { pos, rot }
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

    let mut points = Vec::new();
    for i in 0..NUM_COL {
        for h in 0..NUM_COL {
            let pos = Vector3::new(
                i as f32 * POINT_SIZE - (SIZE as f32 / 2.0) + POINT_SIZE / 2.0 + MARGIN / 4.0,
                h as f32 * POINT_SIZE - (SIZE as f32 / 2.0) + POINT_SIZE / 2.0 + MARGIN / 4.0,
                0.0 as f32,
            );
            points.push(Point {
                pos,
                rot: Vector3::new(0.0, 0.0, 0.0),
            });
        }
    }
    Model { _window, points }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let tick = app.elapsed_frames();
    let secs = tick as f32 / 60.0 / 10.0;
    for p in model.points.iter_mut() {
        p.rot = Vector3::new(
            (secs * p.pos[0]).cos(),
            (secs * p.pos[1]).cos(),
            (secs * p.pos[2]).cos(),
        )
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for p in model.points.iter() {
        draw.ellipse()
            .x_y(p.pos[0], p.pos[1])
            .w_h(POINT_SIZE / 1.2, POINT_SIZE / 1.2)
            .x_radians(p.rot[0])
            .y_radians(p.rot[1]);
        // .z_radians(p.rot[2]);
    }
    draw.to_frame(app, &frame).unwrap();
}
