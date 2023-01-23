# qeelibrs
An original (albeit simple) state vector simulator implementation in Rust.

## Library Usage

Quantum `States` are made up of `Kets`. A `Ket` takes a complex coefficient, an initial value and an entanglement vector.

```
fn create_ket() -> Ket {
    let imaginary_coeff = super::coefficient::create_coefficient(1.0, true);
    let real_coeff = super::coefficient::create_coefficient(1.0, false);
    let complex_coeff = super::coefficient::create_complex_coefficient(real_coeff, imaginary_coeff);

    let init_val = BitVec::from_elem(3, false);
    let ket = super::ket::create_ket(complex_coeff, init_val.clone(), vec![]);
    ket
}
```

A `State` can be composed of any number of `Kets`.

```
kets = vec![create_ket(), create_ket()];
num_qubits = 2;
symbol = 'q';

state = super::state::create_state(kets, num_qubits, symbol);
state.normalize();

assert_eq!(state.num_qubits, num_qubits);
assert_eq!(state.symbol, symbol);
```

An `Ensemble` of `States` can be worked with together in a data structure designed for interacting with
algorithms distributed across multiple distinct (processor) states.

```
let num_qubits = 3;

let first_symbol = 'p';
let second_symbol = 'q';
let first_state = super::state::create_state(vec![create_ket(), create_ket(), create_ket()], num_qubits, first_symbol);
let second_state = super::state::create_state(vec![create_ket(), create_ket(), create_ket()], num_qubits, second_symbol);

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
```

The 

## Executing QASM

A QASM program can be executed using `parser::execute_qasm(source:&str) -> BTreeMap<char, BTreeMap<usize, usize>>` or a RabbitMQ consumer can be started by running `cargo run` which will listen for qasm povided via the queue.

## License

Copyright 2019 Marcus Edwards

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at:

```
http://www.apache.org/licenses/LICENSE-2.0
```

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
