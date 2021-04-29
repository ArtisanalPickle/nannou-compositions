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
    for i in -40..40 {
        let y = i as f32 * 10.0 + (1.0 * app.time * 0.5 * i as f32).sin();
        let points = (-50..50).map(|j| {
            let x = j as f32 * 8.0 + (0.05 * app.time * y as f32).sin() * 3.0;
            Vector2::from((x, y))
        });
        draw.polyline()
            .color(WHITE)
            .weight(2.0 + (1.0 * app.time * 0.5 * i as f32).sin())
            .points(points);
    }
    draw.to_frame(app, &frame).unwrap();
}
