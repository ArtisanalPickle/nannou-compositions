use nannou::prelude::*;

const NPOINTS: usize = 200;
const RADIUS: f32 = 100.0;
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points_colored: Vec<(Point2, Srgba)>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    // let mut points_colored: Vec<(Point2, Srgba)> = vec![];
    let points_colored: Vec<(Point2, Srgba)> = (0..NPOINTS)
        .map(|i| {
            let rad: f32 = i as f32 / NPOINTS as f32 / (2.0 * PI);
            let x = rad.cos() * RADIUS;
            let y = rad.sin() * RADIUS;
            let point = pt2(x, y);
            (point, rgba(1.0, 1.0, 1.0, 1.0))
        })
        .collect();

    // println!("{:?}", &points_colored);

    Model {
        _window,
        points_colored: points_colored,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    draw.polyline()
        .weight(20.0)
        .points_colored(&mut model.points_colored.clone().into_iter());

    draw.to_frame(app, &frame).unwrap();
}
