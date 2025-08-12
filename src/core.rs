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
