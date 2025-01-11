use std::{
    io,
    collections::HashMap,
};

#[derive(Clone)]
struct Rotor {
    wiring: Vec<usize>,
    position: usize,
}

impl Rotor {

    fn new(wiring: &str) -> Self {
        let wiring = wiring
            .chars()
            .map(|c| (c as u8 - b'A') as usize)
            .collect();
        Self { wiring, position: 0 }
    }

    fn encrypt(&self, input: usize) -> usize {
        let index = (input + self.position) % 26;
        self.wiring[index]
    }

    fn decrypt(&self, input: usize) -> usize {
        let index = self.wiring.iter().position(|&x| x == input).unwrap();
        (index + 26 - self.position) % 26
    }

    fn rotate(&mut self) {
        self.position = (self.position + 1) % 26;
    }

}

#[derive(Clone)]
struct Reflector {
    wiring: HashMap<usize, usize>,
}

impl Reflector {

    fn new(wiring: &str) -> Self {
        let mut map = HashMap::new();
        for (i, c) in wiring.chars().enumerate() {
            let index = (c as u8 - b'A') as usize;
            map.insert(i, index);
            map.insert(index, i);
        }
        Self { wiring: map }
    }

    fn reflect(&self, input: usize) -> usize {
        *self.wiring.get(&input).unwrap()
    }

}

struct Enigma {
    rotors: Vec<Rotor>,
    reflector: Reflector,
}

impl Enigma {

    fn new(rotors: Vec<Rotor>, reflector: Reflector) -> Self {
        Self { rotors, reflector }
    }

    fn encrypt_char(&mut self, c: char) -> char {
        let mut index = (c as u8 - b'A') as usize;
        
        for rotor in &self.rotors {
            index = rotor.encrypt(index);
        }
        
        index = self.reflector.reflect(index);
        
        for rotor in self.rotors.iter().rev() {
            index = rotor.decrypt(index);
        }
        
        self.rotors[0].rotate();
        (b'A' + index as u8) as char
    }
    
    fn encrypt(&mut self, text: &str) -> String {
        text.chars().map(|c| self.encrypt_char(c)).collect()
    }
    
    fn decrypt(&mut self, text: &str) -> String {
        text.chars().map(|c| self.encrypt_char(c)).collect()
    }

}

fn main() {
    let mut input = String::new();
    println!("Enter the message to encrypt:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let message = input.trim().to_uppercase();
    
    let mut rotor_wiring1 = String::new();
    println!("Enter rotor 1 wiring:");
    io::stdin().read_line(&mut rotor_wiring1).expect("Failed to read input");
    
    let mut rotor_wiring2 = String::new();
    println!("Enter rotor 2 wiring:");
    io::stdin().read_line(&mut rotor_wiring2).expect("Failed to read input");
    
    let mut reflector_wiring = String::new();
    println!("Enter reflector wiring:");
    io::stdin().read_line(&mut reflector_wiring).expect("Failed to read input");
    
    let rotor1 = Rotor::new(rotor_wiring1.trim());
    let rotor2 = Rotor::new(rotor_wiring2.trim());
    let reflector = Reflector::new(reflector_wiring.trim());
    
    let mut enigma = Enigma::new(vec![rotor1.clone(), rotor2.clone()], reflector.clone());
    let encrypted = enigma.encrypt(&message);
    println!("Encrypted: {}", encrypted);
    
    let mut enigma_decrypt = Enigma::new(vec![rotor1, rotor2], reflector);
    let decrypted = enigma_decrypt.decrypt(&encrypted);
    println!("Decrypted: {}", decrypted);
}
