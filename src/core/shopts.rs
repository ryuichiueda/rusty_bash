//SPDXFileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDXLicense-Identifier: BSD-3-Clause

use std::collections::HashMap;

#[derive(Debug)]
pub struct Shopts {
    opts: HashMap<String, bool>,
}

impl Shopts {
    pub fn new() -> Shopts {
        let mut shopts = Shopts {
            opts: HashMap::new(),
        };

        /*
        let opt_strs = vec!["autocd", "cdable_vars", "cdspell", "checkhash",
                   "checkjobs", "checkwinsize", "cmdhist", "compat31",
                   "compat32", "compat40", "compat41", "dirspell",
                   "dotglob", "execfail", "expand_aliases", "extdebug",
                   "extglob", "extquote", "failglob", "force_fignore",
                   "globstar", "gnu_errfmt", "histappend", "histreedit",
                   "histverify", "hostcomplete", "huponexit", "interactive_comments",
                   "lastpipe", "lithist", "login_shell", "mailwarn",
                   "no_empty_cmd_completion", "nocaseglob", "nocasematch", "nullglob",
                   "progcomp", "promptvars", "restricted_shell", "shift_verbose",
                   "sourcepath", "xpg_echo"];

        for opt in opt_strs {
            shopts.opts.insert(opt.to_string(), false);
        }*/

        shopts.opts.insert("extglob".to_string(), true);

        shopts
    }

    pub fn format(opt: &str, onoff: bool) -> String {
        let onoff_str = match onoff {
            true  => "on",
            false => "off",
        };

        match opt.len() < 16 {
            true  => format!("{:16}{}", opt, onoff_str),
            false => format!("{}\t{}", opt, onoff_str), 
        }
    }

    pub fn print_opt(&self, opt: &str) -> bool {
        match self.opts.get_key_value(opt) {
            None     => {
                eprintln!("sush: shopt: {}: invalid shell option name", opt);
                false
            },
            Some(kv) => {
                println!("{}", Self::format(kv.0, *kv.1));
                true
            },
        }
    }

    pub fn print_all(&self) {
        let mut list = self.opts.iter()
                       .map(|opt| Self::format(opt.0, *opt.1))
                       .collect::<Vec<String>>();

        list.sort();
        list.iter().for_each(|e| println!("{}", e));
    }

    pub fn print_if(&self, onoff: bool) {
        let mut list = self.opts.iter()
                       .filter(|opt| *opt.1 == onoff)
                       .map(|opt| Self::format(opt.0, *opt.1))
                       .collect::<Vec<String>>();

        list.sort();
        list.iter().for_each(|e| println!("{}", e));
    }

    pub fn query(&self, opt: &str) -> bool {
        self.opts[opt]
    }

    pub fn set(&mut self, opt: &str, onoff: bool) -> bool {
        if ! self.opts.contains_key(opt) {
            eprintln!("sush: shopt: {}: invalid shell option name", opt);
            return false;
        }

        self.opts.insert(opt.to_string(), onoff);
        true
    }
}
