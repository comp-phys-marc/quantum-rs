extern crate rand;
use rand::Rng;
use crate::ket::Ket;
use crate::coefficient;
use crate::coefficient::ComplexCoefficient;

#[derive(Clone)]
pub struct State {
    pub kets: Vec<Ket>,
    pub num_qubits: usize,
    pub symbol: char
}

pub fn create_state(kets:Vec<Ket>, num_qubits:usize, symbol:char) -> State {
    State{kets: kets, num_qubits: num_qubits, symbol: symbol}
}

impl State {
        
    pub fn add_ket(&mut self, ket:Ket) {
        self.kets.push(ket);
    }
        
    pub fn remove_ket(&mut self, ket:Ket) {
        match self.kets.iter().position(|k| k.equals(ket.clone())) {
            Some(index) => { self.kets.remove(index); },
            _ => panic!("attempt to remove non-existent ket")
        }
    }
    
    pub fn get_components(&self, qubit:usize) -> [ComplexCoefficient; 2] {
        let empty_coefficient = coefficient::create_coefficient(0.0, false);
        let mut beta = coefficient::create_complex_coefficient(empty_coefficient, empty_coefficient);
        let mut alpha = coefficient::create_complex_coefficient(empty_coefficient, empty_coefficient);
        for ket in &self.kets {
            if Some(true) == ket.get_val().get(qubit) {
                beta = beta.add_to_complex_coefficient(ket.get_coefficient());
            }
            else {
                alpha = alpha.add_to_complex_coefficient(ket.get_coefficient());
            }
        }
        [alpha, beta]
    }

    pub fn x(&mut self, qubit:usize) {
        for ket in &mut self.kets {
            // no-print print!("x ({})", qubit);
            // no-print ket.print();
            // no-print print!(" =");
            ket.x(qubit);
            // no-print ket.print();
            // no-print println!();
        }
    }
         
    pub fn cx(&mut self, source:usize, target:usize) {
        for ket in &mut self.kets {
            // no-print print!("cx ({} -> {})",source, target);
            // no-print ket.print();
            // no-print print!(" =");
            ket.cx(source, target);
            // no-print ket.print();
            // no-print println!();
        }
    }
       
    pub fn y(&mut self, qubit:usize) {
        for ket in &mut self.kets {
            // no-print print!("y ({})", qubit);
            // no-print ket.print();
            // no-print print!(" =");
            ket.y(qubit);
            // no-print ket.print();
            // no-print println!();
        }
    }
            
    pub fn z(&mut self, qubit:usize) {
        for ket in &mut self.kets {
            // no-print print!("z ({})", qubit);
            // no-print ket.print();
            // no-print print!(" =");
            ket.z(qubit);
            // no-print ket.print();
            // no-print println!();
        }
    }
            
    pub fn h(&mut self, qubit:usize) {
        let empty_coefficient = coefficient::create_coefficient(0.0, false);
        let empty_imaginary_coefficient = coefficient::create_coefficient(0.0, true);
        let mut beta = coefficient::create_complex_coefficient(empty_coefficient, empty_imaginary_coefficient);
        let mut alpha = coefficient::create_complex_coefficient(empty_coefficient, empty_imaginary_coefficient);
        let mut one_kets:Vec<Ket> = vec![];
        let mut zero_kets:Vec<Ket> = vec![];

        for ket in &self.kets {
            match ket.get_val().get(qubit) {
                Some(true) => {
                    one_kets.push(ket.clone());
                    beta = beta.add_to_complex_coefficient(ket.get_coefficient()); 
                },
                _ => {
                    zero_kets.push(ket.clone());
                    alpha = alpha.add_to_complex_coefficient(ket.get_coefficient());
                }
            };
        }

        let mut negative_beta = beta.clone();
        negative_beta.negate_magnitude();

        if alpha.equals_complex_coefficient(beta) {
            // no-print print!("h ({})", qubit);
            // no-print for ket in &self.kets {
                // no-print ket.print();
            // no-print }
            // no-print print!(" =");
            self.kets = zero_kets;
            // no-print for ket in &self.kets {
                // no-print ket.print();
            // no-print }
            // no-print println!();
        }

        else if alpha.equals_complex_coefficient(negative_beta) {
            // no-print print!("h ({})", qubit);
            // no-print for ket in &self.kets {
                // no-print ket.print();
            // no-print }
            // no-print print!(" =");
            self.kets = one_kets;
            // no-print for ket in &self.kets {
                // no-print ket.print();
            // no-print }
            // no-print println!();
        }

        else {
            let mut new_kets:Vec<Ket> = vec![];
            for ket in &mut self.kets {
                // no-print print!("h ({})", qubit);
                // no-print ket.print();
                let hadamard_result = ket.h(qubit);
                for result in &hadamard_result {
                    new_kets.push(result.clone());
                }
                // no-print print!(" =");
                // no-print for result in &hadamard_result {
                    // no-print result.print();
                // no-print }
                // no-print println!();
            }
            self.kets = new_kets;
        }
    }
    
    pub fn m(&mut self, qubit:usize) -> bool {
        let empty_coefficient = coefficient::create_coefficient(0.0, false);
        let empty_imaginary_coefficient = coefficient::create_coefficient(0.0, true);
        let mut beta = coefficient::create_complex_coefficient(empty_coefficient, empty_imaginary_coefficient);
        let mut alpha = coefficient::create_complex_coefficient(empty_coefficient, empty_imaginary_coefficient);

        let mut one_kets:Vec<Ket> = vec![];
        let mut zero_kets:Vec<Ket> = vec![];

        for ket in &self.kets {
            match ket.get_val().get(qubit) {
                Some(true) => { 
                    one_kets.push(ket.clone());
                    beta = beta.add_to_complex_coefficient(ket.get_coefficient()); 
                },
                _ => {
                    zero_kets.push(ket.clone());
                    alpha = alpha.add_to_complex_coefficient(ket.get_coefficient());
                }
            };
        }
        let result = self._measure(alpha.to_probability(), beta.to_probability());
        if result {
            self.kets = one_kets;
        }
        else {
            self.kets = zero_kets;
        }
        result
    }
    
    pub fn normalize(&mut self) {
        let mut total_probability = 0.0;
        let mut unique_kets:Vec<Ket> = vec![];

        for ket in &self.kets {
            let mut already_found = false;
            for unique_ket in &unique_kets {
                if ket.get_val() == unique_ket.get_val() {
                    already_found = true;
                    unique_ket.get_coefficient().add_to_complex_coefficient(ket.get_coefficient());
                }
            }
            if !already_found {
                unique_kets.push(ket.clone());
            }
        }
        for unique_ket in &unique_kets {
            total_probability += unique_ket.get_probability();
        }
        let norm_factor = 1.0/((total_probability as f64).sqrt());
        if total_probability != 1.0 {
            for unique_ket in &unique_kets {
                unique_ket.get_coefficient().multiply_by_number(norm_factor);
            }
        }
        self.kets = unique_kets;
        println!("normalizing factor: {}", norm_factor);
    }


    pub fn _measure(&self, alpha:f64, beta:f64) -> bool {
        let cutoff = alpha*100.0;
        let mut rng = rand::thread_rng();
        let outcome:f64 = rng.gen();
        if outcome < cutoff {
            false
        }
        else {
            true
        }
    }
        
    pub fn print(&self) {
        // no-print print!("|{}> =", self.symbol);
        // no-print for ket in &self.kets {
            // no-print ket.print();
        // no-print }
        // no-print println!();
    }

    pub fn print_state_vectors(&self) {
        let mut qubit = 0;

        while qubit < self.num_qubits {
            // no-print println!("qubit {} state vector:", qubit);
            let vector = self.get_components(qubit);
            // no-print print!("alpha: ");

            // no-print for comp in &vector {
                // no-print comp.print();
                // no-print println!();
            // no-print }
            // no-print println!();

            qubit += 1;
        }
    }
}