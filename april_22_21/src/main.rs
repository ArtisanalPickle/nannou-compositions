use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1200, 1200)
        .view(view)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for i in -50..50 {
        let points = (-50..50).map(|j| {
            let x = j as f32 * 8.0 + (0.5 * app.time / j as f32).sin();
            let y = i as f32 * 8.0 + (1.0 * app.time * i as f32).sin();
            Vector2::from((x, y))
        });
        draw.polyline().color(WHITE).weight(1.5).points(points);
    }
    draw.to_frame(app, &frame).unwrap();
}
