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

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    } else {
        draw.rect()
            .w_h(app.window_rect().w(), app.window_rect().h())
            .color(srgba(0.0, 0.0, 0.0, 0.01));
    }

    let cuboid = Cuboid::from_xyz_whd(
        Vector3::new(
            app.time.sin() * 100.0,
            app.time.sin() * 100.0,
            100.0 + app.time.sin() * 100.0,
        ),
        Vector3::new(100.0, 100.0, 100.0),
    );
    let tris = cuboid.triangles_iter().map(|tri| {
        tri.map_vertices(|v| {
            let red = map_range(v.x.sin() * 100.0, -50.0, 50.0, 0.0, 0.8);
            let green = map_range(v.y.sin() * 100.0, -50.0, 50.0, 0.0, 0.8);
            let blue = map_range(v.z.sin() * 100.0, -50.0, 50.0, 0.0, 0.8);
            (v, srgba(red, green, blue, 1.0))
        })
    });

    draw.mesh()
        .tris_colored(tris)
        .x_radians(app.time)
        .y_radians(app.time)
        .z_radians(app.time);
    draw.to_frame(app, &frame).unwrap();
}
