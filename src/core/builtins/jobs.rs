//SPDX-FileCopyrightText: 2024 Ryuichi Ueda <ryuichiueda@gmail.com>
//SPDX-License-Identifier: BSD-3-Clause

use crate::ShellCore;

pub fn jobs(core: &mut ShellCore, _: &mut Vec<String>) -> i32 {
    for (i, job) in core.job_table.iter().enumerate() {
        job.print(i);
    }
    0
}