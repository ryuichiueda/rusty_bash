//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::elements::subword::Subword;
use crate::elements::word::Word;

enum BraceType {
    Comma,
    Range,
}

fn after_dollar(s: &str) -> bool {
    s == "$" || s == "$$"
}

pub fn eval(word: &mut Word) -> Vec<Word> {
    invalidate_brace(&mut word.subwords);

    let mut skip_until = 0;
    for i in word.scan_pos("{") {
        if i < skip_until { //ブレース展開の終わりまで処理をスキップ
             continue;
        }

        if let Some(d) = parse(&word.subwords[i..]) {
            let shift_d: Vec<usize> = d.0.iter().map(|e| e+i).collect();

            if i > 0 && after_dollar(word.subwords[i-1].get_text()) {
                skip_until = *shift_d.last().unwrap();
                continue;
            }

            return match d.1 {
                BraceType::Comma => expand_comma_brace(&word.subwords, &shift_d),
                BraceType::Range => expand_range_brace(&word.subwords, &shift_d),
            }
        }
    }

    vec![word.clone()]
}

fn invalidate_brace(subwords: &mut Vec<Box<dyn Subword>>) {
    if subwords.len() < 2 {
        return;
    }

    if subwords[0].get_text() == "{"
    && subwords[1].get_text() == "}" {
        subwords.remove(1);
        subwords[0].set_text("{}");
    }
}

fn parse(subwords: &[Box<dyn Subword>]) -> Option<(Vec<usize>, BraceType)> {
    let mut stack = vec![];
    for sw in subwords {
        stack.push(Some(sw.get_text()));
        if sw.get_text() == "}" {
            match get_delimiters(&mut stack) {
                Some(found) => return Some(found),
                _           => {},
            }
        }
    }
    None
}

fn get_delimiters(stack: &mut Vec<Option<&str>>) -> Option<(Vec<usize>, BraceType)> {
    let mut comma_pos = vec![];
    let mut period_pos = vec![];

    for i in (1..stack.len()-1).rev() {
        if stack[i] == Some(",") {
            comma_pos.push(i);
        } else if stack[i] == Some(".") {
            period_pos.push(i);
        }else if stack[i] == Some("{") { // find an inner brace expcomma_posion
            stack[i..].iter_mut().for_each(|e| *e = None);
            return None;
        }
    }

    if comma_pos.len() > 0 {
        comma_pos.reverse();
        comma_pos.insert(0, 0); // add "{" pos
        comma_pos.push(stack.len()-1); // add "}" pos
        return Some( (comma_pos, BraceType::Comma) );
    }

    if period_pos.len() > 1 && period_pos[0] == period_pos[1] + 1 {
        period_pos.reverse();
        period_pos.insert(0, 0);
        period_pos.push(stack.len()-1);
        return Some( (period_pos, BraceType::Range) );
    }
    None
}

fn expand_comma_brace(subwords: &Vec<Box<dyn Subword>>, delimiters: &Vec<usize>) -> Vec<Word> {
    let left = &subwords[..delimiters[0]];
    let mut right = subwords[(delimiters.last().unwrap()+1)..].to_vec();
    invalidate_brace(&mut right);

    let mut ans = vec![];
    let mut from = delimiters[0] + 1;
    for to in &delimiters[1..] {
        let mut w = Word::new();
        w.subwords.extend(left.to_vec());
        w.subwords.extend(subwords[from..*to].to_vec());
        w.subwords.extend(right.to_vec());
        w.text = w.subwords.iter().map(|s| s.get_text()).collect();
        ans.append(&mut eval(&mut w));
        from = *to + 1;
    }
    ans
}

fn expand_range_brace(subwords: &Vec<Box<dyn Subword>>, delimiters: &Vec<usize>) -> Vec<Word> {
    let left = &subwords[..delimiters[0]];
    let mut right = subwords[(delimiters.last().unwrap()+1)..].to_vec();
    invalidate_brace(&mut right);

    let len = delimiters.len();
    let start = &subwords[delimiters[0]+1].get_text();
    let end = &subwords[delimiters[len-1]-1].get_text();

    let start_num = match start.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return expand_range_brace_failure(subwords),
    };
    let end_num = match end.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return expand_range_brace_failure(subwords),
    };

    if start_num <= end_num {
        let mut ans = vec![];
        let mut sw = subwords[delimiters[0]+1].clone();

        let mut w = Word::new();
        sw.set_text(&start_num.to_string());
        w.subwords.extend(left.to_vec());
        w.subwords.push(sw.clone());
        ans.push(w);

        for n in (start_num+1)..end_num {
            sw.set_text(&n.to_string());
            let mut w = Word::new();
            w.subwords.push( sw.clone() );
            w.text = n.to_string();

            ans.push(w);
        }

        let mut w = Word::new();
        sw.set_text(&end_num.to_string());
        w.subwords.push(sw.clone());
        w.subwords.extend(right.to_vec());
        ans.push(w);

        return ans;
    }

    expand_range_brace_failure(subwords)
}

fn expand_range_brace_failure(subwords: &Vec<Box<dyn Subword>>) -> Vec<Word> {
    let mut w = Word::new();
    w.subwords = subwords.to_vec();
    w.text = w.subwords.iter().map(|s| s.get_text()).collect();
    vec![w]
}
