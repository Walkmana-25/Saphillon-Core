// Sapphillon-Core
// Copyright 2025 Yuta Takahashi
//
// This file is part of Sapphillon-Core
//
// Sapphillon-Core is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::runtime::{OpStateWorkflowData, WorkflowStdout};
use deno_core::{OpState, op2};
use std::io::{Write, stderr, stdout};
use std::sync::{Arc, Mutex};

#[op2(fast)]
pub(crate) fn op_print_wrapper(
    state: &mut OpState,
    #[string] msg: &str,
    is_err: bool,
) -> Result<(), std::io::Error> {
    let mut data = state
        .borrow_mut::<Arc<Mutex<OpStateWorkflowData>>>()
        .lock()
        .unwrap();

    if is_err {
        if data.is_capture_stdout() {
            // data.add_result(WorkflowStdout::Stderr(msg.to_string()));
            data.add_result(WorkflowStdout::Stdout(msg.to_string()));
        } else {
            stderr().write_all(msg.as_bytes())?;
            stderr().flush().unwrap();
        }
    } else if data.is_capture_stdout() {
        data.add_result(WorkflowStdout::Stdout(msg.to_string()));
    } else {
        stdout().write_all(msg.as_bytes())?;
        stdout().flush().unwrap();
    }

    Ok(())
}
