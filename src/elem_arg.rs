//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::ShellCore;
use crate::utils::{eval_glob, combine};
use crate::debuginfo::DebugInfo;
use crate::Feeder;
use crate::abst_arg_elem::*;
use crate::abst_arg_elem::ArgElem;
use crate::elem_subarg_tilde::SubArgTildeUser;
use crate::elem_subarg_non_quoted::SubArgNonQuoted;
use crate::abst_command_elem::CommandElem;

pub struct Arg {
    pub text: String,
    pub pos: DebugInfo,
    pub subargs: Vec<Box<dyn ArgElem>>
}

impl Arg {
    pub fn new() -> Arg {
        Arg {
            text: "".to_string(),
            pos: DebugInfo{lineno: 0, pos: 0, comment: "".to_string()},
            subargs: vec!(),
        }
    }

    pub fn expand_glob(text: &String) -> Vec<String> {
        let mut ans = eval_glob(text);

        if ans.len() == 0 {
            let s = text.clone().replace("\\*", "*").replace("\\\\", "\\");
            ans.push(s);
        };
        ans
    }

    pub fn remove_escape(text: &String) -> String{
        let mut escaped = false;
        let mut ans = "".to_string();

        let deescape_target = |c: char| {
            "$*\" \\`{};()^<>?[]'!".chars().any(|x| x == c)
        };
        
        for ch in text.chars() {
            if escaped || ch != '\\' {
                if escaped && !deescape_target(ch) {
                    ans.push('\\');
                }
                ans.push(ch);
            };
            escaped = !escaped && ch == '\\';
        }
        ans
    }

    // single quoted arg or double quoted arg or non quoted arg 
    pub fn parse(text: &mut Feeder, expand_brace: bool, conf: &mut ShellCore) -> Option<Arg> {
        if text.len() == 0 {
            return None;
        }

        let mut ans = Arg{
            text: "".to_string(),
            pos: DebugInfo::init(text),
            subargs: vec!(),
        };

        if let Some(result) = SubArgTildeUser::parse(text, false) {
            ans.text += &result.text();
            ans.subargs.push(Box::new(result));
        }
    
        let sub = if expand_brace{subarg}else{subvalue};
    
        while let Some(result) = sub(text, conf) {
            ans.text += &(*result).text();
            ans.subargs.push(result);
    
            if text.len() == 0 {
                break;
            };
        };
    
        if ans.text.len() > 0 {
            Some(ans)
        }else{
            None
        }
    }
}

impl CommandElem for Arg {
    fn parse_info(&self) -> Vec<String> {
        let mut ans = vec!(format!("    arg      : '{}' ({})",
                              self.text.clone(), self.pos.text()));
        for sub in &self.subargs {
            ans.push("        subarg      : ".to_owned() + &*sub.text());
        };

        ans
    }

    fn eval(&mut self, conf: &mut ShellCore) -> Vec<String> {
        let mut subevals = vec!();
        for sa in &mut self.subargs {
            let vs = sa.eval(conf);

            let mut cvs = vec!();
            if sa.permit_lf(){
                cvs = vs;
            }else{
                for v in vs {
                    let cv = v.iter().map(|s| s.replace("\n", " ")).collect();
                    cvs.push(cv);
                }
            }
            
            subevals.push(cvs);
        }
        //eprintln!("SUBEVALS: {:?}", subevals);

        let mut strings = vec!();

        for ss in subevals {
            strings = combine(&mut strings, ss);
        }
        //eprintln!("STRINGS: {:?}", strings);

        let mut ans = vec!();
        for v in strings {
            ans.append(&mut v.clone());
        }
        //eprintln!("ARGS: {:?}", ans);
        ans
    }

    fn text(&self) -> String { self.text.clone() }
}

pub fn arg_in_brace(text: &mut Feeder, conf: &mut ShellCore) -> Option<Arg> {
    let mut ans = Arg{
        text: "".to_string(),
        pos: DebugInfo::init(text),
        subargs: vec!(),
    };

    let backup = text.clone();
    if text.match_at(0, ",}"){ // zero length arg
        let tmp = SubArgNonQuoted{
            text: "".to_string(),
            pos: DebugInfo::init(text),
        };
        ans.subargs.push(Box::new(tmp));
        return Some(ans);
    };

    if let Some(result) = SubArgTildeUser::parse(text, true) {
        ans.text += &result.text();
        ans.subargs.push(Box::new(result));
    }

    while let Some(result) = subarg_in_brace(text, conf) {
        ans.text += &(*result).text();
        ans.subargs.push(result);
    };

    if text.len() == 0 ||  !text.match_at(0, ",}"){ 
        text.rewind(backup);
        return None;
    }

    Some(ans)
}
