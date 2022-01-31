use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    debug: bool,
    shapes: Vec<Shape>,
}

struct Shape {
    x: f32,
    y: f32,
    r: f32,
}

impl Shape {
    fn draw(&self, app_draw: &Draw) {
        app_draw
            .ellipse()
            .color(GREEN)
            .w(self.r)
            .h(self.r)
            .x_y(self.x, self.y);
    }

    fn new() -> Shape {
        Shape {
            x: 10.,
            y: 25.,
            r: 300.,
        }
    }
}

fn model(_app: &App) -> Model {
    Model {
        debug: true,
        shapes: vec![Shape::new()],
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);
    let draw = app.draw();
    draw.ellipse().color(BLUE).w(40.).h(40.).x_y(10., 10.);

    for shape in &model.shapes {
        shape.draw(&draw);
        // Write to the window frame.
        draw.to_frame(app, &frame).unwrap();
    }
}
