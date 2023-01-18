extern crate qasm;

use std::collections::BTreeMap;
use rustsimulationservice::parser::execute_qasm;

#[test]
fn test_lexer() {
    let source = r#"
    OPENQASM 2.0;
    qreg a[3];
    CX a[0], a[1];
    "#;

    let tokens = qasm::lex(source);
    assert_eq!(
        vec![
            qasm::Token::OpenQASM,
            qasm::Token::Real(2.0),
            qasm::Token::Semicolon,
            qasm::Token::QReg,
            qasm::Token::Id("a".to_string()),
            qasm::Token::LSParen,
            qasm::Token::NNInteger(3),
            qasm::Token::RSParen,
            qasm::Token::Semicolon,
            qasm::Token::Id("CX".to_string()),
            qasm::Token::Id("a".to_string()),
            qasm::Token::LSParen,
            qasm::Token::NNInteger(0),
            qasm::Token::RSParen,
            qasm::Token::Comma,
            qasm::Token::Id("a".to_string()),
            qasm::Token::LSParen,
            qasm::Token::NNInteger(1),
            qasm::Token::RSParen,
            qasm::Token::Semicolon
        ],
        tokens
    )
}

#[test]
fn test_parser() {
    let source = r#"
    OPENQASM 2.0;
    qreg q[3];
    qreg r[3];
    x q[0];
    cx q[0], q[1];
    creg c[3];
    measure q[0]->c[0];
    measure r[0]->c[1];
    measure q[0]->c[2];
    "#;

    let result = execute_qasm(source);
    let mut expect = BTreeMap::new();
    let mut regs = BTreeMap::new();
    regs.insert(0, 1);
    regs.insert(1, 0);
    regs.insert(2, 1);
    expect.insert('c', regs);
    assert_eq!(result, expect);
}