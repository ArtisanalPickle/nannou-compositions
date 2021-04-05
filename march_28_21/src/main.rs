use nannou::math::Float;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

const SIZE: f32 = 3.0;
const SPACE: f32 = 10.0;

fn view(app: &App, _model: &Model, frame: Frame) {
    let row = (app.window_rect().h() / (SIZE as f32 + SPACE * 2.0)) as i32;
    let col = (app.window_rect().w() / (SIZE as f32 + SPACE * 2.0)) as i32;
    let draw = app.draw();
    let time = app.elapsed_frames() / 5;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    } else {
        draw.rect()
            .w_h(app.window_rect().w(), app.window_rect().h())
            .color(srgba(0.0, 0.0, 0.0, 0.075));
    }

    for i in -row..row + 1 {
        for j in -col..col + 1 {
            let rad = (time as f32 + i as f32 + (j as f32 + time as f32).sin()) / 4.0;
            let x = i as f32 * (SIZE + SPACE);
            let y = j as f32 * (SIZE + SPACE);
            let hypot = ((x.powi(2) + y.powi(2)).sqrt()).sin();
            let scale = (rad / 2.0).sin() + hypot;
            draw.ellipse()
                .color(WHITE)
                .x_y(x, y)
                .w_h(scale * SIZE, scale * SIZE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
