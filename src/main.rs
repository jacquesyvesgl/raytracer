use rtiow::scenes::*;
use rtiow::render::render;

fn main() {
    let scene = three_balls();
    render(scene, "zebi.ppm");
}
