struct Rotor {
    wiring: [char; 26],
    position: usize,
}

impl Rotor {
    fn new(wiring: [char; 26], position: usize) -> Rotor {
        Rotor { wiring, position }
    }
}

fn main() {
    println!("Hello, world!");
}
