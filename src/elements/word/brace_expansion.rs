//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::elements::subword::Subword;
use super::Word;

pub fn eval(word: &mut Word) -> Vec<Word> {
    for i in open_brace_pos(word) {
        if let Some(d) = parse(&word.subwords[i..], i) {
            return expand(&word.subwords, &d);
        }
    }
    vec![word.clone()]
}

fn open_brace_pos(w: &Word) -> Vec<usize> {
    w.subwords.iter()
        .enumerate()
        .filter(|e| e.1.get_text() == "{")
        .map(|e| e.0)
        .collect()
}

pub fn parse(subwords: &[Box<dyn Subword>], start: usize) -> Option<Vec<usize>> {
    let mut stack = vec![];
    for sw in subwords {
        stack.push(Some(sw.get_text()));
        if sw.get_text() == "}" {
            match get_delimiters(&mut stack, start) {
                Some(ds) => return Some(ds),
                _        => {},
            }
        }
    }
    None
}

fn get_delimiters(stack: &mut Vec<Option<&str>>, start: usize) -> Option<Vec<usize>> {
    let mut comma_pos = vec![start, stack.len()-1+start];
    for i in (1..stack.len()-1).rev() {
        if stack[i] == Some(",") {
            comma_pos.insert(1, start+i);
        }else if stack[i] == Some("{") { // find an inner brace expcomma_posion
            stack[i..].iter_mut().for_each(|e| *e = None);
            return None;
        }
    }

    match comma_pos.len() {
        2 => None,
        _ => Some(comma_pos),
    }
}

pub fn expand(subwords: &Vec<Box<dyn Subword>>, delimiters: &Vec<usize>) -> Vec<Word> {
    let left = &subwords[..delimiters[0]];
    let right = &subwords[(delimiters.last().unwrap()+1)..];

    let mut ans = vec![];
    let mut from = delimiters[0] + 1;
    for to in &delimiters[1..] {
        let mut w = Word::new();
        w.subwords = [ left, &subwords[from..*to], right ].concat();
        w.text = w.subwords.iter().map(|s| s.get_text()).collect();
        ans.append(&mut eval(&mut w));
        from = *to + 1;
    }
    ans
}
