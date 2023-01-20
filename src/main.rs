use rtiow::scenes::*;
use rtiow::render::render;

fn main() {
    let scene = cornell_box();
    render(scene, "zebi.ppm");
}
