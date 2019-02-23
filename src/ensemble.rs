use std::collections::BTreeMap;
use crate::state::State;
use crate::ket;

pub struct Ensemble {
    pub subsystems: BTreeMap<char, State>
}

pub fn create_ensemble(subsystems:BTreeMap<char, State>) -> Ensemble {
    Ensemble{subsystems: subsystems}
}

impl Ensemble {

    pub fn add_subsystem(&mut self, state:State, name:char) {
        self.subsystems.insert(name, state);
    }

    pub fn m(&mut self, target_system:char, target_qubit:usize) -> bool {

        let outcome = match self.subsystems.get(&target_system) {
            Some(system) => system.m(target_qubit),
            None => panic!("attempt to measure non-existent system")
        };
        
        for (name, subsystem) in &mut self.subsystems {
            for entangled_ket in &subsystem.kets {
                if entangled_ket.is_entangled() && entangled_ket.is_entangled_with(target_system, target_qubit) {
                    let collapse = entangled_ket.should_collapse(outcome, target_system, target_qubit);
                    if collapse {
                        subsystem.remove_ket(*entangled_ket);
                    }
                }
            }
        }
        outcome
    }

    pub fn cx(&mut self, source_system:char, source_qubit:usize, target_system:char, target_qubit:usize) {

        let mut source = match self.subsystems.get(&source_system) {
            Some(source) => source,
            None => panic!("attempt to control from non-existent system")
        };

        let mut target = match self.subsystems.get(&target_system) {
            Some(target) => *target,
            None => panic!("attempt to control to non-existent system")
        };

        if source_system == target_system {
            source.cx(source_qubit, target_qubit);
        }
        else {
            let [alpha_source, beta_source] = source.get_components(source_qubit);

            print!("q: ");
            source.print();

            for ket in &mut target.kets {
                print!("cx ({}[{}] -> {}[{}])", source_system, source_qubit, target_system, target_qubit);
                ket.print();
                print!(" =");

                let new_coeff = ket.get_coefficient();
                let new_val = ket.get_val();
                let mut new_ket = ket::create_ket(new_coeff, new_val, vec![]);
                new_ket.x(target_qubit);

                new_ket.set_coefficient(new_ket.get_coefficient().multiply_by_complex_coefficient(alpha_source));
                ket.set_coefficient(ket.get_coefficient().multiply_by_complex_coefficient(beta_source));

                new_ket.entangle(false, source_system, source_qubit);
                target.add_ket(new_ket);

                ket.entangle(true, source_system, source_qubit);
                ket.print();
                println!();

                target.print();
            }
        }
    }
}
