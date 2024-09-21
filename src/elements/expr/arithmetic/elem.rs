//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use super::ArithmeticExpr;
use super::Word;

#[derive(Debug, Clone)]
pub enum Elem {
    UnaryOp(String),
    BinaryOp(String),
    Integer(i64),
    Float(f64),
    Ternary(Box<Option<ArithmeticExpr>>, Box<Option<ArithmeticExpr>>),
    Word(Word, i64), // Word + post increment or decrement
    InParen(ArithmeticExpr),
    Increment(i64), //pre increment
    Delimiter(String), //delimiter dividing left and right of &&, ||, and ','
}

pub fn op_order(op: &Elem) -> u8 {
    match op {
        Elem::Increment(_) => 20,
        Elem::UnaryOp(s) => {
            match s.as_str() {
                "-" | "+" => 19,
                _         => 18,
            }
        },
        Elem::BinaryOp(s) => {
            match s.as_str() {
                "**"            => 17, 
                "*" | "/" | "%" => 16, 
                "+" | "-"       => 15, 
                "<<" | ">>"     => 14, 
                "<=" | ">=" | ">" | "<" => 13, 
                "==" | "!="     => 12, 
                "&"             => 11, 
                "^"             => 10, 
                "|"             => 9, 
                "&&"             => 8, 
                "||"             => 7, 
                ","             => 0, 
                _               => 2, //substitution
            }
        },
        Elem::Ternary(_, _) => 3,
        _ => 1, 
    }
}

pub fn to_string(op: &Elem) -> String {
    match op {
        Elem::InParen(a) => a.text.to_string(),
        Elem::Integer(n) => n.to_string(),
        Elem::Float(f) => f.to_string(),
        Elem::Word(w, inc) => {
            match inc {
                1  => w.text.clone() + "++",
                -1 => w.text.clone() + "--",
                _  => w.text.clone(),
            }
        },
        Elem::UnaryOp(s) => s.clone(),
        Elem::BinaryOp(s) => s.clone(),
        Elem::Increment(1) => "++".to_string(),
        Elem::Increment(-1) => "--".to_string(),
        _ => "".to_string(),
    }
}