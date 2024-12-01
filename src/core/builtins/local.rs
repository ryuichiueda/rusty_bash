//SPDX-FileCopyrightText: 2024 Ryuichi Ueda <ryuichiueda@gmail.com>
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use crate::utils::exit;
use crate::core::data::Value;
use crate::elements::substitution::Substitution;

fn set(arg: &str, core: &mut ShellCore, layer: usize) -> bool {
    let mut feeder = Feeder::new(arg);
    if feeder.scanner_name(core) == feeder.len() { // name only
        let name = feeder.consume(feeder.len());
        return core.data.set_layer_param(&name, "", layer);
    }

    let mut sub = match Substitution::parse(&mut feeder, core) {
        Some(s) => s,
        _ => {
            eprintln!("sush: local: `{}': not a valid identifier", arg);
            return false;
        },
    };

    match sub.eval(core) {
        true => {},
        false => exit::internal("unsupported substitution"),
    }

    match sub.evaluated_value {
        Value::EvaluatedSingle(s) => core.data.set_layer_param(&sub.key, &s, layer),
        Value::EvaluatedArray(a)  => core.data.set_layer_array(&sub.key, &a, layer),
        _ => exit::internal("unsupported substitution"),
    }
}

pub fn local(core: &mut ShellCore, args: &mut Vec<String>) -> i32 {
    let layer = if core.data.get_layer_num() > 2 {
        core.data.get_layer_num() - 2 //The last element of data.parameters is for local itself.
    }else{
        eprintln!("sush: local: can only be used in a function");
        return 1;
    };

    match args[1..].iter().all(|a| set(a, core, layer)) {
        true  => 0,
        false => 1,
    }
}
