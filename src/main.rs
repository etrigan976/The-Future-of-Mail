/*
    local crate imports
*/
use nannou::prelude::*;
/*
    global data
*/
struct Model {}
/// # main function
/// This function initializes the nannou framework app
fn main() {
    let mut _running: bool = true;
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}
/// if this function has an error the window will turn purple
fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(PURPLE);
}
