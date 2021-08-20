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

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(rgba(0.85, 0.8, 0.8, 1.0));
    let n_points = 500;
    let vertices = (0..n_points)
        .map(|p| pt2(p as f32, p as f32))
        .enumerate()
        .map(|(i, p)| {
            (
                p,
                srgba(app.mouse.x / win.right() - (p[0] / win.w()), 0.0, 0.0, 1.0),
            )
        });
    draw.polyline().weight(20.0).points_colored(vertices);

    draw.to_frame(app, &frame).unwrap();
}
