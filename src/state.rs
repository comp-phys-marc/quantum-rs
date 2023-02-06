//! # State
//! A data structure that represents a full quantum state and maintains a set of underlying kets.

extern crate rand;
use rand::Rng;
use crate::ket::Ket;
use crate::coefficient;
use crate::coefficient::ComplexCoefficient;
use crate::coefficient::Coefficient;
use crate::coefficient::FloatCoefficient;

#[derive(Clone)]
pub enum Backend {
    X86, // swaps out the coefficient impl
    WASM, // swaps out the coefficient impl
    RS, // leaves all impl usage as-is
    QMASM, // swaps out the coefficient impl and uses wasm_pfc
    QASM, // swaps out the ket impl
    DELEGATED  // swaps out the ket impl and uses publisher
}

#[derive(Clone)]
pub struct State {
    pub kets: Vec<Ket>,
    pub num_qubits: usize,
    pub symbol: char,
    pub backend: Backend,
    pub lazy: bool,
    pub verbose: bool
}

/// Initializes a quantum state with a given set of kets and number of qubits.
pub fn create_state(kets:Vec<Ket>, num_qubits:usize, symbol:char, backend:Option<Backend>, lazy:Option<bool>, verbose:Option<bool>) -> State {
    State{kets: kets, num_qubits: num_qubits, symbol: symbol, backend: backend.unwrap_or(Backend::RS), lazy: lazy.unwrap_or(false), verbose: verbose.unwrap_or(false)}
}

impl State {

    /// Adds a ket to the overall quantum state.
    pub fn add_ket(&mut self, ket:Ket) {
        self.kets.push(ket);
    }

    /// Removes a ket from the overall quantum state.        
    pub fn remove_ket(&mut self, ket:Ket) {
        match self.kets.iter().position(|k| k.equals(ket.clone())) {
            Some(index) => { self.kets.remove(index); },
            _ => panic!("attempt to remove non-existent ket")
        }
    }

    /// Determines the components of the state vector for the given target qubit.    
    pub fn get_components(&self, qubit:usize) -> [ComplexCoefficient; 2] {
        let empty_coefficient = coefficient::create_coefficient(0.0, false);
        let empty_imaginary_coefficient = coefficient::create_coefficient(0.0, true);
        let mut beta = coefficient::create_complex_coefficient(empty_coefficient, empty_imaginary_coefficient);
        let mut alpha = coefficient::create_complex_coefficient(empty_coefficient, empty_imaginary_coefficient);
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

    /// Performs a Pauli X gate on the target qubit.
    pub fn x(&mut self, qubit:usize) {
        for ket in &mut self.kets {
            if self.verbose {
                print!("x ({})", qubit);
                ket.print();
                print!(" =");
            }
            ket.x(qubit);
            if self.verbose {
                ket.print();
                println!();
            }
        }
    }
         
    /// Performs a Controlled X gate on the target qubit with the 
    /// source qubit as controller.
    pub fn cx(&mut self, source:usize, target:usize) {
        for ket in &mut self.kets {
            if self.verbose {
                print!("cx ({} -> {})",source, target);
                ket.print();
                print!(" =");
            }
            ket.cx(source, target);
            if self.verbose {
                ket.print();
                println!();
            }
        }
    }

    /// Performs a Pauli Y gate on the target qubit.       
    pub fn y(&mut self, qubit:usize) {
        for ket in &mut self.kets {
            if self.verbose {
                print!("y ({})", qubit);
                ket.print();
                print!(" =");
            }
            ket.y(qubit);
            if self.verbose {
                ket.print();
                println!();
            }
        }
    }

    /// Performs a Pauli Z gate on the target qubit.            
    pub fn z(&mut self, qubit:usize) {
        for ket in &mut self.kets {
            if self.verbose {
                print!("z ({})", qubit);
                ket.print();
                print!(" =");
            }
            ket.z(qubit);
            if self.verbose {
                ket.print();
                println!();
            }
        }
    }

    /// Performs a Hadamard gate on the target qubit.
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
            if self.verbose {
                print!("h ({})", qubit);
                for ket in &self.kets {
                    ket.print();
                }
                print!(" =");
            }
            self.kets = zero_kets;
            if self.verbose {
                for ket in &self.kets {
                    ket.print();
                }
                println!();
            }
        }

        else if alpha.equals_complex_coefficient(negative_beta) {
            if self.verbose {
                print!("h ({})", qubit);
                for ket in &self.kets {
                    ket.print();
                }
                print!(" =");
            }
            self.kets = one_kets;
            if self.verbose {
                for ket in &self.kets {
                    ket.print();
                }
                println!();
            }
        }

        else {
            let mut new_kets:Vec<Ket> = vec![];
            for ket in &mut self.kets {
                if self.verbose {
                    print!("h ({})", qubit);
                    ket.print();
                }
                let hadamard_result = ket.h(qubit);
                for result in &hadamard_result {
                    new_kets.push(result.clone());
                }
                if self.verbose {
                    print!(" =");
                    for result in &hadamard_result {
                        result.print();
                    }
                    println!();
                }
            }
            self.kets = new_kets;
        }
    }

    /// Measures the target qubit.    
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

    /// Normalizes the current quantum state.    
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
        if self.verbose {
            println!("normalizing factor: {}", norm_factor);
        }
    }


    /// Used pseudo-random number generation to simulate the probabilistic outcome of a qubit 
    /// measurement. Update the quantum system with the measurement results.
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

    /// Prints the full quantum state.        
    pub fn print(&self) {
        print!("|{}> =", self.symbol);
        for ket in &self.kets {
            ket.print();
        }
        println!();
    }

    /// Prints the state vector for each qubit.
    pub fn print_state_vectors(&self) {
        let mut qubit = 0;

        while qubit < self.num_qubits {
            println!("qubit {} state vector:", qubit);
            let vector = self.get_components(qubit);
            print!("alpha: ");

            for comp in &vector {
                comp.print();
                println!();
            }
            println!();

            qubit += 1;
        }
    }
}