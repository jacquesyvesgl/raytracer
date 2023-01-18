use rtiow::scenes::*;
use rtiow::render::render;

fn main() {

    let scene = get_three_balls();
    // let image = render(&scene);
    render(scene, "zebi.ppm");
}
