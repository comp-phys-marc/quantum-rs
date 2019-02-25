use std::collections::BTreeMap;
use std::mem::replace;
use crate::state::State;
use crate::ket::Ket;
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

        let system = match self.subsystems.get(&target_system) {
            Some(system) => system,
            None => panic!("attempt to measure non-existent system")
        };

        let outcome = system.clone().m(target_qubit);

        let mut to_remove:Vec<Ket> = vec![];
        
        for (name, subsystem) in &mut self.subsystems {
            to_remove = vec![];
            for entangled_ket in &mut subsystem.kets {
                if entangled_ket.is_entangled() && entangled_ket.is_entangled_with(target_system, target_qubit) {
                    let collapse = entangled_ket.should_collapse(outcome, target_system, target_qubit);
                    if collapse {
                        to_remove.push(entangled_ket.clone());
                    }
                }
            }
            let mut new_subsystem = subsystem.clone();
            for ket in to_remove {
                new_subsystem.remove_ket(ket);
            }
            replace(subsystem, new_subsystem);
        }
        outcome
    }

    pub fn cx(&mut self, source_system:char, source_qubit:usize, target_system:char, target_qubit:usize) {

        let mut source = match self.subsystems.get(&source_system) {
            Some(source) => source,
            None => panic!("attempt to control from non-existent system")
        };

        let mut new_source = source.clone();

        let mut target = match self.subsystems.get(&target_system) {
            Some(target) => target,
            None => panic!("attempt to control to non-existent system")
        };

        let mut new_target = target.clone();

        if source_system == target_system {
            new_source.cx(source_qubit, target_qubit);
        }
        else {
            let [alpha_source, beta_source] = source.get_components(source_qubit);

            // no-print print!("q: ");
            // no-print source.print();

            let mut new_kets:Vec<Ket>  = vec![];

            for ket in &mut new_target.kets {
                // no-print print!("cx ({}[{}] -> {}[{}])", source_system, source_qubit, target_system, target_qubit);
                // no-print ket.print();
                // no-print print!(" =");

                let new_coeff = ket.get_coefficient();
                let new_val = ket.get_val();
                let mut new_ket = ket::create_ket(new_coeff, new_val, vec![]);
                new_ket.x(target_qubit);

                new_ket.set_coefficient(new_ket.get_coefficient().multiply_by_complex_coefficient(alpha_source));
                ket.set_coefficient(ket.get_coefficient().multiply_by_complex_coefficient(beta_source));

                new_ket.entangle(false, source_system, source_qubit);
                new_kets.push(new_ket.clone());

                ket.entangle(true, source_system, source_qubit);
                // no-print ket.print();
                // no-print println!();
            }

            for ket in new_kets {
                new_target.add_ket(ket);
                // no-print new_target.print();
            }
            replace(&mut target, &new_target);
            replace(&mut source, &new_source);
        }
    }
}
