use std::mem::replace;
extern crate bit_vec;
use bit_vec::BitVec;
use crate::coefficient::ComplexCoefficient;



#[derive(Clone)]
pub struct Ket {
    coefficient: ComplexCoefficient,
    val: BitVec,
    entanglements: Vec<Entanglement>
}

pub fn create_ket(coeff:ComplexCoefficient, val:BitVec, entanglements:Vec<Entanglement>) -> Ket {
    Ket{coefficient: coeff, val: val, entanglements: entanglements}
}

impl Ket {
        
    pub fn equals(&self, other:Ket) -> bool {
        self.val == other.get_val()
    }
        
    pub fn get_val(&self) -> BitVec {
        self.val.clone()
    }
    
    pub fn get_coefficient(&self) -> ComplexCoefficient {
        self.coefficient
    }
        
    pub fn set_val(&mut self, val:BitVec) {
        self.val = val;
    }
                    
    pub fn set_coefficient(&mut self, coeff:ComplexCoefficient) {
        self.coefficient = coeff;
    }

    pub fn get_probability(&self) -> f64 {
        self.coefficient.to_probability()
    }

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
    
    pub fn cx(&mut self, source:usize, target:usize) {
        if self.val.get(source) == Some(true) { 
            self.x(target);
        }
    }
    
    pub fn z(&mut self, qubit:usize) {
        if self.val.get(qubit) == Some(true) {
            self.coefficient.negate_magnitude()
        }
    }
    
    pub fn y(&mut self, qubit:usize) {
        self.z(qubit);
        self.x(qubit);
        self.coefficient.multiply_by_i();
    }
        
    pub fn print(&self) {
        // no-print self.coefficient.print();
        // no-print print!("|{:?}>", self.val);
    }

    pub fn is_entangled(&self) -> bool {
        self.entanglements.len() > 0
    }

    pub fn is_entangled_with(&self, system:char, qubit:usize) -> bool {
        let mut is_entangled = false;
        for entanglement in &self.entanglements {
            if entanglement.get_system() == system && entanglement.get_qubit() == qubit {
                is_entangled = true
            }
        }
        is_entangled
    }

    pub fn entangle(&mut self, outcome:bool, system:char, qubit:usize) {
        self.entanglements.push(create_entanglement(outcome, system, qubit));
    }

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

    pub fn copy_entanglement_to(&self, mut other_ket:Ket) {
        for entanglement in &self.entanglements {
            other_ket.entangle(entanglement.get_outcome(), entanglement.get_system(), entanglement.get_qubit())
        }
    }

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

pub fn create_entanglement(outcome:bool, system:char, qubit:usize) -> Entanglement {
    Entanglement{outcome: outcome, system: system, qubit: qubit}
}

impl Entanglement {

    pub fn get_system(&self) -> char {
        self.system
    }

    pub fn get_qubit(&self) -> usize {
        self.qubit
    }

    pub fn get_outcome(&self) -> bool {
        self.outcome
    }
}