//! # Ket
//! Data structures that represent the information associated with a single ket in a quantum state,
//! including any relationship to entanglement and non-linear effects.

extern crate bit_vec;
use bit_vec::BitVec;
use crate::coefficient::ComplexCoefficient;

#[derive(Clone)]
pub struct Ket {
    coefficient: ComplexCoefficient,
    val: BitVec,
    entanglements: Vec<Entanglement>
}

/// Initializes a ket with a value and coefficient.
pub fn create_ket(coeff:ComplexCoefficient, val:BitVec, entanglements:Vec<Entanglement>) -> Ket {
    Ket{coefficient: coeff, val: val, entanglements: entanglements}
}

impl Ket {

    /// The equality of kets compares their qubit strings, not their coefficients.        
    pub fn equals(&self, other:Ket) -> bool {
        self.val == other.get_val()
    }

    /// The qubit string value of the ket.        
    pub fn get_val(&self) -> BitVec {
        self.val.clone()
    }

    /// Returns the complex coefficient of the ket.    
    pub fn get_coefficient(&self) -> ComplexCoefficient {
        self.coefficient
    }

    /// Sets the qubit string value for the ket.        
    pub fn set_val(&mut self, val:BitVec) {
        self.val = val;
    }

    /// Sets the coefficient of the ket's term in the overall quantum state.                    
    pub fn set_coefficient(&mut self, coeff:ComplexCoefficient) {
        self.coefficient = coeff;
    }

    /// Determines the probabilistic weight of the ket within its overall 
    /// quantum state.
    pub fn get_probability(&self) -> f64 {
        self.coefficient.to_probability()
    }

    /// Performs a Pauli X gate on the target qubit.
    pub fn x(&mut self, qubit:usize) {
        let mut q:usize = 0;
        while q < self.val.len() {
            if q == qubit {
                match self.val.get(q) {
                    Some(mut val) => { 
                        if val == true {
                            self.val.set(q, false);
                        }
                        else {
                            self.val.set(q, true);
                        }
                     },
                    None => panic!("attempt to flip non-existent qubit.")
                };
                break;
            }
            q += 1;
        }
    }
    
    /// Performs a Controlled X gate on the target qubit with the source qubit
    /// as controller.
    pub fn cx(&mut self, source:usize, target:usize) {
        if self.val.get(source) == Some(true) { 
            self.x(target);
        }
    }

    /// Performs a Pauli Z gate on the target qubit.
    pub fn z(&mut self, qubit:usize) {
        if self.val.get(qubit) == Some(true) {
            self.coefficient.negate_magnitude()
        }
    }

    /// Performs a Pauli Y gate on the target qubit.    
    pub fn y(&mut self, qubit:usize) {
        self.z(qubit);
        self.x(qubit);
        self.coefficient.multiply_by_i();
    }

    /// Prints the state.        
    pub fn print(&self) {
        self.coefficient.print();
        print!("|{:?}>", self.val);
    }

    /// Determines whether the existence of the ket is predicated upon entanglement 
    /// interactions.
    pub fn is_entangled(&self) -> bool {
        self.entanglements.len() > 0
    }


    /// Determines whether the existence of the ket is predicated upon the entanglement 
    /// interaction of the given qubit.
    pub fn is_entangled_with(&self, system:char, qubit:usize) -> bool {
        let mut is_entangled = false;
        for entanglement in &self.entanglements {
            if entanglement.get_system() == system && entanglement.get_qubit() == qubit {
                is_entangled = true
            }
        }
        is_entangled
    }

    /// Registers the ket's dependence on an entanglement of qubits.
    pub fn entangle(&mut self, outcome:bool, system:char, qubit:usize) {
        self.entanglements.push(create_entanglement(outcome, system, qubit));
    }

    /// Determines whether the ket should collapse and disappear following a measurement 
    /// of the given qubit.
    pub fn should_collapse(&mut self, outcome:bool, system:char, qubit:usize) -> bool {
        let mut should_collapse = false;
        let mut remove_index:isize = -1;
        for (index, entanglement) in self.entanglements.iter().enumerate() {
            if entanglement.get_system() == system && entanglement.get_qubit() == qubit {
                remove_index = index as isize;
                if entanglement.get_outcome() != outcome {
                    should_collapse = true
                }
            }
        }
        if remove_index > 0 {
            self.entanglements.remove(remove_index as usize);
        }
        should_collapse
    }

    /// Copies the entanglement reference to another ket. This is to be
    /// used when operating on a ket which will disappear when an entangled
    /// qubit is measured such that a new ket is created.
    pub fn copy_entanglement_to(&self, mut other_ket:Ket) {
        for entanglement in &self.entanglements {
            other_ket.entangle(entanglement.get_outcome(), entanglement.get_system(), entanglement.get_qubit())
        }
    }

    /// Performs a Hadamard gate on the target qubit.
    pub fn h(&mut self, qubit:usize) -> [Ket; 2] {
        let mut new_ket = create_ket(self.coefficient, self.val.clone(), self.entanglements.clone());
        new_ket.x(qubit);

        if self.val.get(qubit) == Some(true) {
            self.coefficient.negate_magnitude();
        }
        [create_ket(self.get_coefficient(), self.get_val(), vec![]), new_ket]
    }
}

#[derive(Clone)]
pub struct Entanglement {
    outcome:bool,
    system:char,
    qubit:usize
}

/// Initializes an entanglement object.
pub fn create_entanglement(outcome:bool, system:char, qubit:usize) -> Entanglement {
    Entanglement{outcome: outcome, system: system, qubit: qubit}
}

impl Entanglement {

    /// Gets the system owning the entangled qubit.
    pub fn get_system(&self) -> char {
        self.system
    }

    /// Gets the qubit which is entangled.
    pub fn get_qubit(&self) -> usize {
        self.qubit
    }

    /// Gets the outcome for which the ket will not disappear.
    pub fn get_outcome(&self) -> bool {
        self.outcome
    }
}