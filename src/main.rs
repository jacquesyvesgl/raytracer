use rtiow::scenes::*;
use rtiow::render::render_and_write;

fn main() {

    let scene = get_three_balls();
    // let image = render(&scene);
    render_and_write(&scene, "zebi.ppm");

    // write_image(image, &scene, "zebi.ppm");
}
