extern crate bit_vec;

use bit_vec::BitVec;
use std::collections::BTreeMap;

use crate::ket::Ket;
use crate::state::State;
use crate::parser::execute_qasm;
use crate::coefficient::{ Coefficient, FloatCoefficient, ComplexCoefficient };

fn create_ket() -> Ket {
    let imaginary_coeff: FloatCoefficient = super::coefficient::create_coefficient(1.0, true);
    let real_coeff: FloatCoefficient = super::coefficient::create_coefficient(1.0, false);
    let complex_coeff: ComplexCoefficient = super::coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);
    let init_state = BitVec::from_elem(3, false);
    let ket: Ket = super::ket::create_ket(complex_coeff, init_state.clone(), vec![]);
    ket
}

#[test]
fn test_create_coefficient() {
    let mut magnitude:f64 = 1.00;
    let mut imaginary:bool = true;
    let mut coeff: FloatCoefficient = super::coefficient::create_coefficient(magnitude, imaginary);
    assert_eq!(coeff.get_magnitude(), magnitude);
    assert_eq!(coeff.get_imaginary(), imaginary);

    magnitude = 0.50;
    imaginary = false;
    coeff = super::coefficient::create_coefficient(magnitude, imaginary);
    assert_eq!(coeff.get_magnitude(), magnitude);
    assert_eq!(coeff.get_imaginary(), imaginary);
}

#[test]
fn test_equals_coefficient() {
    let coeff: FloatCoefficient = super::coefficient::create_coefficient(1.0, true);
    let other: FloatCoefficient = super::coefficient::create_coefficient(1.0, true);
    assert!(coeff.equals_coefficient(other));

    let different = super::coefficient::create_coefficient(2.0, false);
    assert!(!coeff.equals_coefficient(different));
}

#[test]
fn test_create_ket() {
    let mut imaginary_coeff: FloatCoefficient = super::coefficient::create_coefficient(1.0, true);
    let mut real_coeff: FloatCoefficient = super::coefficient::create_coefficient(1.0, false);
    let mut complex_coeff: ComplexCoefficient = super::coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);
    let mut init_state = BitVec::from_elem(3, false);
    let mut ket: Ket = super::ket::create_ket(complex_coeff, init_state.clone(), vec![]);
    assert!(ket.get_coefficient().equals_complex_coefficient(complex_coeff));
    assert_eq!(ket.get_val(), init_state);

    imaginary_coeff = super::coefficient::create_coefficient(0.0, true);
    real_coeff = super::coefficient::create_coefficient(0.5, false);
    complex_coeff = super::coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);
    init_state = BitVec::from_elem(3, false);
    ket = super::ket::create_ket(complex_coeff, init_state.clone(), vec![]);
    assert!(ket.get_coefficient().equals_complex_coefficient(complex_coeff));
    assert_eq!(ket.get_val(), init_state);
}

#[test]
fn test_create_state() {
    let mut kets: Vec<Ket> = vec![create_ket(), create_ket(), create_ket()];
    let mut num_qubits = 3;
    let mut symbol = 'p';
    let mut state = super::state::create_state(kets, num_qubits, symbol, None, None, None);
    assert_eq!(state.num_qubits, num_qubits);
    assert_eq!(state.symbol, symbol);

    kets = vec![create_ket(), create_ket()];
    num_qubits = 2;
    symbol = 'q';
    state = super::state::create_state(kets, num_qubits, symbol, None, None, None);
    assert_eq!(state.num_qubits, num_qubits);
    assert_eq!(state.symbol, symbol);
}

#[test]
fn test_create_ensemble() {
    let num_qubits = 3;
    let first_symbol = 'p';
    let second_symbol = 'q';
    let first_state = super::state::create_state(vec![create_ket(), create_ket(), create_ket()], num_qubits, first_symbol, None, None, None);
    let second_state = super::state::create_state(vec![create_ket(), create_ket(), create_ket()], num_qubits, second_symbol, None, None, None);
    let mut subsystems:BTreeMap<char, State> = BTreeMap::new();
    subsystems.insert(first_symbol, first_state);
    subsystems.insert(second_symbol, second_state);
    let ensemble = super::ensemble::create_ensemble(subsystems);

    let subsystem_p = match ensemble.subsystems.get(&first_symbol) {
        Some(subsystem_p) => subsystem_p,
        None => panic!("could not retrieve subsystem from ensemble")
    };

    let subsystem_q = match ensemble.subsystems.get(&second_symbol) {
        Some(subsystem_q) => subsystem_q,
        None => panic!("could not retrieve subsystem from ensemble")
    };

    assert_eq!(subsystem_p.symbol, first_symbol);
    assert_eq!(subsystem_q.symbol, second_symbol);
}
