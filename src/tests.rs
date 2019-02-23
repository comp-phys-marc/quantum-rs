extern crate bit_vec;
use bit_vec::BitVec;
use std::collections::BTreeMap;
use crate::ket::Ket;
use crate::state::State;

fn create_ket() -> Ket {
    let imaginary_coeff = super::coefficient::create_coefficient(1.0, true);
    let real_coeff = super::coefficient::create_coefficient(1.0, false);
    let complex_coeff = super::coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);
    let init_state = BitVec::from_elem(3, false);
    let ket = super::ket::create_ket(complex_coeff, init_state.clone(), vec![]);
    ket
}

#[test]
fn test_create_coefficient() {
    let magnitude:f64 = 1.00;
    let imaginary:bool = true;
    let coeff = super::coefficient::create_coefficient(magnitude, imaginary);
    assert_eq!(coeff.get_magnitude(), magnitude);
    assert_eq!(coeff.get_imaginary(), imaginary);
}

#[test]
fn test_equals_coefficient() {
    let coeff = super::coefficient::create_coefficient(1.0, true);
    let other = super::coefficient::create_coefficient(1.0, true);
    assert!(coeff.equals_coefficient(other));

    let different = super::coefficient::create_coefficient(2.0, false);
    assert!(!coeff.equals_coefficient(different));
}

#[test]
fn test_create_ket() {
    let imaginary_coeff = super::coefficient::create_coefficient(1.0, true);
    let real_coeff = super::coefficient::create_coefficient(1.0, false);
    let complex_coeff = super::coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);
    let init_state = BitVec::from_elem(3, false);
    let ket = super::ket::create_ket(complex_coeff, init_state.clone(), vec![]);
    assert!(ket.get_coefficient().equals_complex_coefficient(complex_coeff));
    assert_eq!(ket.get_val(), init_state);
}

#[test]
fn test_create_state() {
    let kets = vec![create_ket(), create_ket(), create_ket()];
    let num_qubits = 3;
    let symbol = 'p';
    let state = super::state::create_state(kets, num_qubits, symbol);
    assert_eq!(state.num_qubits, num_qubits);
    assert_eq!(state.symbol, symbol);
}

#[test]
fn test_create_ensemble() {
    let num_qubits = 3;
    let first_symbol = 'p';
    let second_symbol = 'q';
    let first_state = super::state::create_state(vec![create_ket(), create_ket(), create_ket()], num_qubits, first_symbol);
    let second_state = super::state::create_state(vec![create_ket(), create_ket(), create_ket()], num_qubits, second_symbol);
    let mut subsystems:BTreeMap<char, State> = BTreeMap::new();
    subsystems.insert(first_symbol, first_state);
    subsystems.insert(second_symbol, second_state);
    let ensemble = super::ensemble::create_ensemble(subsystems);
}
