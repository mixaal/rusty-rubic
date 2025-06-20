use rusty_rubic::cube;

fn main() {
    let mut cube = cube::RubicsCube::new();
    cube.show();
    cube.rotate_up(2);
    cube.rotate_left(2);
    cube.rotate_right(2);
    cube.rotate_down(2);
    cube.show();
}
