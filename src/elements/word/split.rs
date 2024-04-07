//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::ShellCore;
use crate::elements::word::Word;
use crate::elements::subword::{Subword, SubwordType};
use nix::unistd::User;

pub fn eval(word: &Word, core: &mut ShellCore) -> Vec<Word> {
    let mut ans = vec![];
    for (i, sw) in word.subwords.iter().enumerate() {
        let split = sw.split(core);
        if split.len() == 1 {
            continue;
        }
        
        ans.append(&mut rearrange(word, split, i));
    }

    if ans.len() == 0 {
        vec![word.clone()]
    }else{
        ans
    }
}

fn rearrange(word: &Word, subwords: Vec<Box<dyn Subword>>, pos: usize) -> Vec<Word> {
    let mut ans = vec![];
    let split_len = subwords.len();

    let mut left = Word::new();
    if pos != 0 {
        left.subwords = word.subwords[..pos].to_vec();
    }
    left.subwords.push(subwords[0].clone());
    ans.push(left);

    for sw in subwords[1..split_len-1].iter() {
        let mut mid = Word::new();
        mid.subwords.push(sw.clone());
        ans.push(mid);
    }

    let mut right = Word::new();
    right.subwords.push(subwords[split_len-1].clone());
    if pos != word.subwords.len()-1 {
        right.subwords = word.subwords[pos+1..].to_vec();
    }

    ans.push(right);

    ans
}