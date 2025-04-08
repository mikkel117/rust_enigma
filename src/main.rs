struct Rotor {
    wiring: [u8; 26],
    position: usize,
}

impl Rotor {
    fn new(wiring: [u8; 26], position: usize) -> Rotor {
        Rotor { wiring, position }
    }

    fn step(&mut self) {
        self.position = (self.position + 1) % 26;
    }

    fn encode_forward(&self, c: char) -> char {
        let index = (c as u8 - b'A' + self.position as u8) % 26;
        let mapped_index = self.wiring[index as usize];
        ((mapped_index + 26 - self.position as u8) % 26 + b'A') as char
    }

    fn encode_backward(&self, c: char) -> char {
        let index = (c as u8 - b'A' + self.position as u8) % 26;
        let mapped_index = self.wiring.iter().position(|&x| x == index).unwrap() as u8; // Find reverse mapping
        ((mapped_index + 26 - self.position as u8) % 26 + b'A') as char
    }
}

fn main() {
    let rotor = Rotor::new(
        [
            9, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17,
            2, 4,
        ],
        0,
    );

    let ouutput1 = rotor.encode_forward('A');
    let ouutput2 = rotor.encode_backward(ouutput1);

    
}
