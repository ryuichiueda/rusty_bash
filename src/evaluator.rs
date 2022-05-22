//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use nix::unistd::{execvp, fork, ForkResult, Pid}; 
use nix::sys::wait::*;
use std::ffi::CString;
use std::process::exit;
use crate::ShellCore;
use crate::evaluator_args::Arg;
use crate::utils::blue_string;
use crate::debuginfo::DebugInfo;

pub trait BashElem {
    fn parse_info(&self) -> Vec<String>;
    fn exec(&self, _conf: &mut ShellCore){}
    fn eval(&self, _conf: &mut ShellCore) -> Vec<String> { vec!() }
    fn text(&self) -> String { String::new() }
}

/* delimiter */
#[derive(Debug)]
pub struct ArgDelimiter {
    pub text: String,
    pub debug: DebugInfo,
}

impl BashElem for ArgDelimiter {
    fn parse_info(&self) -> Vec<String> {
        vec!(format!("    delimiter: '{}' ({})", self.text.clone(), self.debug.text()))
    }
}

#[derive(Debug)]
pub struct Eoc {
    pub text: String,
    pub debug: DebugInfo,
}

impl BashElem for Eoc {
    fn parse_info(&self) -> Vec<String> {
        vec!(format!("    end mark : '{}' ({})\n", self.text.clone(), self.debug.text()))
    }
}

#[derive(Debug)]
pub struct Quote {
    pub text: String,
    pub debug: DebugInfo,
}

impl BashElem for Quote {
    fn parse_info(&self) -> Vec<String> {
        vec!(format!("    quote : '{}' ({})\n", self.text.clone(), self.debug.text()))
    }
}

/* command: delim arg delim arg delim arg ... eoc */
pub struct CommandWithArgs {
    pub elems: Vec<Box<dyn BashElem>>,
    pub text: String,
    //pub debug: DebugInfo,
}

impl BashElem for CommandWithArgs {
    fn parse_info(&self) -> Vec<String> {
        let mut ans = vec!(format!("command: '{}'", self.text));
        for elem in &self.elems {
            ans.append(&mut elem.parse_info());
        };
        
        blue_string(&ans)
    }

    fn exec(&self, conf: &mut ShellCore){
        let mut args = self.eval_args(conf);

        if let Some(func) = conf.get_internal_command(&args[0]) {
            func(&mut args);
            return;
        }

        unsafe {
            match fork() {
                Ok(ForkResult::Child) => self.exec_external_command(&args, conf),
                Ok(ForkResult::Parent { child } ) => CommandWithArgs::wait_command(child),
                Err(err) => panic!("Failed to fork. {}", err),
            }
        }
    }
}

impl CommandWithArgs {
    fn eval_args(&self, conf: &mut ShellCore) -> Vec<String> {
        let mut args = vec!();

        for elem in &self.elems {
            for s in &elem.eval(conf) {
                args.append(&mut Arg::expand_glob(&s.clone()));
            }
        };

        args.iter()
            .map(|a| Arg::remove_escape(&a))
            .collect()
    }

    fn exec_external_command(&self, args: &Vec<String>, conf: &mut ShellCore) {
        let cargs: Vec<CString> = args
            .iter()
            .map(|a| CString::new(a.to_string()).unwrap())
            .collect();

        if conf.flags.d {
            for s in self.parse_info() {
                eprintln!("{}", s);
            };
        };

        if let Ok(_) = execvp(&cargs[0], &*cargs){
        }

        eprintln!("Command not found");
        exit(127);
    }

    fn wait_command(child: Pid) {
        match waitpid(child, None)
            .expect("Faild to wait child process.") {
            WaitStatus::Exited(pid, status) => {
                if status != 0 {
                    println!("Pid: {:?}, Exit with {:?}", pid, status);
                };
            }
            WaitStatus::Signaled(pid, signal, _) => {
                println!("Pid: {:?}, Signal: {:?}", pid, signal)
            }
            _ => {
                println!("Unknown error")
            }
        };
    }
}
