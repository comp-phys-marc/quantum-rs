extern crate bit_vec;
extern crate qasm;

use crate::ket;
use crate::ket::Ket;
use crate::state;
use crate::state::State;
use crate::coefficient;
use crate::ensemble;
use crate::ensemble::Ensemble;

use bit_vec::BitVec;
use std::collections::BTreeMap;


pub fn init_ket(num_qubits:usize) -> Ket {
    let imaginary_coeff = coefficient::create_coefficient(1.0, true);
    let real_coeff = coefficient::create_coefficient(1.0, false);
    let complex_coeff = coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);
    let init_state = BitVec::from_elem(num_qubits, false);
    let ket = ket::create_ket(complex_coeff, init_state.clone(), vec![]);
    ket
}

pub fn init_state(num_qubits:usize, symbol:char) -> State {
    let mut kets = Vec::new();

    let mut i = 0;
    while i < num_qubits {
        kets.push(init_ket(num_qubits.clone()));
        i += 1;
    }

    let state = state::create_state(kets, num_qubits, symbol);
    state
}

pub fn init_ensemble() -> Ensemble {
    let mut subsystems:BTreeMap<char, State> = BTreeMap::new();
    let ensemble = ensemble::create_ensemble(subsystems);
    ensemble
}

pub fn init_classical_reg(size:usize) -> BTreeMap<usize, usize> {
    let reg:BTreeMap<usize, usize> = BTreeMap::new();
    reg
}

pub fn execute_qasm(source:&str) -> BTreeMap<char, BTreeMap<usize, usize>> {

    let mut tokens = qasm::lex(source);
    let mut ensemble:Ensemble = init_ensemble();
    let mut classical_regs:BTreeMap<char, BTreeMap<usize, usize>> = BTreeMap::new();

    println!("QASM received...");

    match qasm::parse(&mut tokens) {
        Ok(nodes) => {
            let mut ast_nodes = nodes.clone();

            for mut ast_node in ast_nodes {
                match ast_node {
                    qasm::AstNode::QReg(ref mut identifier, ref mut size) => {
                        let id:Vec<char> = identifier.chars().collect();
                        let state = init_state(*size as usize, id[0]);
                        ensemble.subsystems.insert(id[0], state);
                    },
                    qasm::AstNode::CReg(ref mut identifier, ref mut size) => {
                        let id:Vec<char> = identifier.chars().collect();
                        let reg = init_classical_reg(*size as usize);
                        classical_regs.insert(id[0], reg);
                    },
                    qasm::AstNode::Measure(ref mut source, ref mut dest) => {
                        let mut classical_index:usize = 0;
                        let mut classical_reg:char = 'c';
                        let mut quantum_index:usize = 0;
                        let mut quantum_reg:char = 'q';
                        let mut proceed = true;

                        match source {
                            qasm::Argument::Qubit(ref mut identifier, ref mut index) => {
                                let id:Vec<char> = identifier.chars().collect();
                                quantum_reg = id[0];
                                quantum_index = *index as usize;
                            }
                            qasm::Argument::Register(ref mut reg) => {
                                println!("Unsupported full register gate on {}... skipping", *reg);
                                proceed = false;
                            }
                            _ => {
                                println!("Parser error in measurement!");
                                proceed = false;
                            }
                        }
                        match dest {
                            qasm::Argument::Qubit(ref mut identifier, ref mut index) => {
                                let id:Vec<char> = identifier.chars().collect();
                                classical_reg = id[0];
                                classical_index = *index as usize;
                            }
                            qasm::Argument::Register(ref mut reg) => {
                                println!("Unsupported full register gate on {}... skipping", *reg);
                                proceed = false;
                            }
                            _ => {
                                println!("Parser error in measurement!");
                                proceed = false;
                            }
                        }
                        if proceed {
                            let mut creg = classical_reg.clone();
                            let result = ensemble.m(quantum_reg, quantum_index);

                            let mut old_val = classical_regs.get(&creg);
                            match old_val {
                                Some(val) => {
                                    let mut new_val = BTreeMap::new();
                                    new_val = val.clone();
                                    new_val.insert(classical_index, result as usize);
                                    classical_regs.insert(creg, new_val.clone());
                                }
                                _ => {}
                            }
                        }
                    },
                    qasm::AstNode::ApplyGate(ref mut name, ref mut qubits, ref mut params) => {

                        if name != "cx" {
                            let mut i:usize = 0;
                            let mut reg = 'q';
                            let mut proceed = true;

                            match qubits[0] {
                                qasm::Argument::Qubit(ref mut identifier, ref mut index) => {
                                    let id:Vec<char> = identifier.chars().collect();
                                    reg = id[0];
                                    i = *index as usize;
                                }
                                qasm::Argument::Register(ref mut reg) => {
                                    println!("Unsupported full register gate on {}... skipping", *reg);
                                    proceed = false;
                                }
                                _ => {
                                    println!("Parser error in measurement!");
                                    proceed = false;
                                }
                            }
                            if proceed {
                                let mut state = ensemble.subsystems.get(&reg);

                                match state {
                                    Some(q) => {
                                        let mut new_state = q.clone();
                                        if name == "h" {
                                            new_state.h(i);
                                        }
                                        if name == "z" {
                                            new_state.z(i);
                                        }
                                        if name == "y" {
                                            new_state.y(i);
                                        }
                                        if name == "x" {
                                            new_state.x(i);
                                        }
                                        ensemble.subsystems.insert(reg, new_state);
                                    }
                                    _ => {}
                                }
                            }
                        }

                        else {
                            let mut first_i:usize = 0;
                            let mut first_reg = 'q';
                            let mut second_i:usize = 0;
                            let mut second_reg = 'q';
                            let mut proceed = true;

                            match qubits[0] {
                                qasm::Argument::Qubit(ref mut identifier, ref mut index) => {
                                    let id:Vec<char> = identifier.chars().collect();
                                    first_reg = id[0];
                                    first_i = *index as usize;
                                }
                                qasm::Argument::Register(ref mut reg) => {
                                    println!("Unsupported full register gate on {}... skipping", *reg);
                                    proceed = false;
                                }
                                _ => {
                                    println!("Parser error in measurement!");
                                    proceed = false;
                                }
                            }
                            match qubits[1] {
                                qasm::Argument::Qubit(ref mut identifier, ref mut index) => {
                                    let id:Vec<char> = identifier.chars().collect();
                                    second_reg = id[0];
                                    second_i = *index as usize;
                                }
                                qasm::Argument::Register(ref mut reg) => {
                                    println!("Unsupported full register gate on {}... skipping", *reg);
                                    proceed = false;
                                }
                                _ => {
                                    println!("Parser error in measurement!");
                                    proceed = false;
                                }
                            }
                            if proceed {
                                ensemble.cx(first_reg, first_i, second_reg, second_i);
                            }
                        }
                    },
                    _ => println!("Skipping unsupported operation"),
                }
            }
        }
        Err(e) => println!("Error parsing qasm: {}", e)
    };
    classical_regs
}
