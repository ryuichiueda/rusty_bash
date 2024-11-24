//SPDX-FileCopyrightText: 2024 Ryuichi Ueda <ryuichiueda@gmail.com>
//SPDX-License-Identifier: BSD-3-Clause

use super::{GlobElem, extglob};

fn eat_one_char(pattern: &mut String, ans: &mut Vec<GlobElem>) -> bool {
    if pattern.starts_with("*") || pattern.starts_with("?") {
        ans.push( GlobElem::Symbol(pattern.remove(0))  );
        return true;
    }
    false
}

fn eat_escaped_char(pattern: &mut String, ans: &mut Vec<GlobElem>) -> bool {
    if ! pattern.starts_with("\\") {
        return false;
    }

    if pattern.len() == 1 {
        ans.push( GlobElem::Normal(pattern.remove(0).to_string()) );
        return true;
    }

    let len = pattern.chars().nth(0).unwrap().len_utf8();
    ans.push( GlobElem::Normal( consume(pattern, len) ) );
    true
}

fn eat_bracket(pattern: &mut String, ans: &mut Vec<GlobElem>) -> bool {
    if ! pattern.starts_with("[") {
        return false;
    }
    
    let mut chars = vec![];
    let mut len = 1;
    let mut escaped = false;
    let mut not = false;

    if pattern.starts_with("[^") || pattern.starts_with("[!") {
        not = true;
        len = 2;
    }

    for c in pattern[len..].chars() {
        len += c.len_utf8();

        if escaped {
            chars.push(c); 
            escaped = false;
        }else if c == '\\' {
            escaped = true;
        }else if c == ']' {
            let expand_chars = expand_range_representation(&chars);
            match not {
                false => ans.push( GlobElem::OneOf(expand_chars) ),
                true  => ans.push( GlobElem::NotOneOf(expand_chars) ),
            }
            consume(pattern, len);
            return true;
        }else{
            chars.push(c);
        }
    }

    false
}

pub fn parse(pattern: &str, extglob: bool) -> Vec<GlobElem> {
    let pattern = pattern.to_string();
    let mut remaining = pattern.to_string();
    let mut ans = vec![];

    while remaining.len() > 0 {
        if extglob {
            let (len, extparen) = extglob::scan(&remaining);
            if len > 0 {
                consume(&mut remaining, len);
                ans.push(extparen.unwrap());
                continue;
            }
        }

        if eat_bracket(&mut remaining, &mut ans) 
        || eat_one_char(&mut remaining, &mut ans) 
        || eat_escaped_char(&mut remaining, &mut ans) {
            continue;
        }

        let len = scan_chars(&remaining);
        if len > 0 {
            let s = consume(&mut remaining, len);
            ans.push( GlobElem::Normal(s) );
            continue;
        }

        let s = consume(&mut remaining, 1);
        ans.push( GlobElem::Normal(s) );
    }

    ans
}

fn scan_chars(remaining: &str) -> usize {
    let mut ans = 0;
    for c in remaining.chars() {
        if "@!+*?[\\".find(c) != None {
            return ans;
        }
        ans += c.len_utf8();
    }
    ans
}

fn expand_range_representation(chars: &Vec<char>) -> Vec<char> {
    let mut ans = vec![];
    let mut from = None;
    let mut hyphen = false;

    for c in chars {
        if *c == '-' {
            hyphen = true;
            continue;
        }

        if hyphen {
            if ans.len() > 0 {
                ans.pop();
            }

            let mut expand = expand_range(&from, c);
            ans.append(&mut expand);
            hyphen = false;
            continue;
        }else {
            ans.push(*c);
            from = Some(*c);
        }
    }

    if hyphen {
        ans.push('-');
    }

    ans
}

fn expand_range(from: &Option<char>, to: &char) -> Vec<char> {
    if from.is_none() {
        return vec![*to];
    }

    let from = from.unwrap();

    let mut ans = vec![];

    if ('0' <= from && from <= *to && *to <= '9')
    || ('a' <= from && from <= *to && *to <= 'z')
    || ('A' <= from && from <= *to && *to <= 'Z') {
        let mut ch = from;
        while ch <= *to {
            ans.push(ch);
            ch = (ch as u8 + 1) as char;
        }

    }
    ans
}

fn consume(remaining: &mut String, cutpos: usize) -> String {
    let cut = remaining[0..cutpos].to_string();
    *remaining = remaining.split_off(cutpos);

    cut
}
