extern crate bit_vec;
use bit_vec::BitVec;
use std::time::Instant;
use std::collections::BTreeMap;
mod ket;
mod state;
mod coefficient;

fn create_eleven_bit_ket() -> ket::Ket {
    let imaginary_coeff = coefficient::create_coefficient(1.0, true);
    let real_coeff = coefficient::create_coefficient(1.0, false);
    let complex_coeff = coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);
    let init_state = BitVec::from_elem(11, false);
    let ket = ket::create_ket(complex_coeff, init_state.clone(), vec![]);
    ket
}

fn ten_qubit_counterfeit_coin_finding() -> BitVec {

    let num_qubits = 10;
    let symbol = 'q';
    let mut state = state::create_state(vec![create_eleven_bit_ket()], num_qubits, symbol);
    let mut creg = BitVec::from_elem(10, false);

    state.h(0);
    state.h(1);
    state.h(2);
    state.h(3);
    state.h(4);
    state.h(5);
    state.h(6);
    state.h(7);
    state.h(8);

    state.cx(0, 10);
    state.cx(1, 10);
    state.cx(2, 10);
    state.cx(3, 10);
    state.cx(4, 10);
    state.cx(5, 10);
    state.cx(6, 10);
    state.cx(7, 10);
    state.cx(8, 10);

    let result = state.m(9);
    creg.set(9, result);

    if result == false { state.x(9); }
    if result == false { state.h(9); }
    if result == true { state.h(0); }
    if result == true { state.h(1); }
    if result == true { state.h(2); }
    if result == true { state.h(3); }
    if result == true { state.h(4); }
    if result == true { state.h(5); }
    if result == true { state.h(6); }
    if result == true { state.h(7); }
    if result == true { state.h(8); }
    if result == false { state.cx(6, 9); }
    if result == false { state.h(0); }
    if result == false { state.h(1); }
    if result == false { state.h(2); }
    if result == false { state.h(3); }
    if result == false { state.h(4); }
    if result == false { state.h(5); }
    if result == false { state.h(6); }
    if result == false { state.h(7); }
    if result == false { state.h(8); }

    state.normalize();

    let mut q = 0;
    while q < 10 {
        creg.set(q, state.m(q));
        q += 1;
    }
    creg
}

fn eleven_qubit_counterfeit_coin_finding() -> BitVec {

    let num_qubits = 11;
    let symbol = 'q';
    let mut state = state::create_state(vec![create_eleven_bit_ket()], num_qubits, symbol);
    let mut creg = BitVec::from_elem(11, false);

    state.h(0);
    state.h(1);
    state.h(2);
    state.h(3);
    state.h(4);
    state.h(5);
    state.h(6);
    state.h(7);
    state.h(8);
    state.h(9);

    state.cx(0, 10);
    state.cx(1, 10);
    state.cx(2, 10);
    state.cx(3, 10);
    state.cx(4, 10);
    state.cx(5, 10);
    state.cx(6, 10);
    state.cx(7, 10);
    state.cx(8, 10);
    state.cx(9, 10);

    let result = state.m(10);
    creg.set(10, result);

    if result == false { state.x(10); }
    if result == false { state.h(10); }
    if result == true { state.h(0); }
    if result == true { state.h(1); }
    if result == true { state.h(2); }
    if result == true { state.h(3); }
    if result == true { state.h(4); }
    if result == true { state.h(5); }
    if result == true { state.h(6); }
    if result == true { state.h(7); }
    if result == true { state.h(8); }
    if result == true { state.h(9); }
    if result == false { state.cx(6, 10); }
    if result == false { state.h(0); }
    if result == false { state.h(1); }
    if result == false { state.h(2); }
    if result == false { state.h(3); }
    if result == false { state.h(4); }
    if result == false { state.h(5); }
    if result == false { state.h(6); }
    if result == false { state.h(7); }
    if result == false { state.h(8); }
    if result == false { state.h(9); }

    state.normalize();

    let mut q = 0;
    while q < 11 {
        creg.set(q, state.m(q));
        q += 1;
    }
    creg
}

fn twelve_qubit_counterfeit_coin_finding() -> BitVec {

    let num_qubits = 12;
    let symbol = 'q';
    let mut state = state::create_state(vec![create_eleven_bit_ket()], num_qubits, symbol);
    let mut creg = BitVec::from_elem(12, false);

    state.h(0);
    state.h(1);
    state.h(2);
    state.h(3);
    state.h(4);
    state.h(5);
    state.h(6);
    state.h(7);
    state.h(8);
    state.h(9);
    state.h(10);


    state.cx(0, 11);
    state.cx(1, 11);
    state.cx(2, 11);
    state.cx(3, 11);
    state.cx(4, 11);
    state.cx(5, 11);
    state.cx(6, 11);
    state.cx(7, 11);
    state.cx(8, 11);
    state.cx(9, 11);
    state.cx(10, 11);

    let result = state.m(11);
    creg.set(11, result);

    if result == false { state.x(10); }
    if result == false { state.h(10); }
    if result == true { state.h(0); }
    if result == true { state.h(1); }
    if result == true { state.h(2); }
    if result == true { state.h(3); }
    if result == true { state.h(4); }
    if result == true { state.h(5); }
    if result == true { state.h(6); }
    if result == true { state.h(7); }
    if result == true { state.h(8); }
    if result == true { state.h(9); }
    if result == false { state.cx(6, 10); }
    if result == false { state.h(0); }
    if result == false { state.h(1); }
    if result == false { state.h(2); }
    if result == false { state.h(3); }
    if result == false { state.h(4); }
    if result == false { state.h(5); }
    if result == false { state.h(6); }
    if result == false { state.h(7); }
    if result == false { state.h(8); }
    if result == false { state.h(9); }

    state.normalize();

    let mut q = 0;
    while q < 11 {
        creg.set(q, state.m(q));
        q += 1;
    }
    creg
}


fn main() {
    let mut start = Instant::now();
    let mut output = ten_qubit_counterfeit_coin_finding();
    let mut elapsed = start.elapsed();
    println!("Elapsed time for 10 qubits: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    start = Instant::now();
    output = eleven_qubit_counterfeit_coin_finding();
    elapsed = start.elapsed();
    println!("Elapsed time for 11 qubits: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
