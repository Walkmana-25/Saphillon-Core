#![warn(clippy::field_reassign_with_default)]

use deno_core::{Extension, JsRuntime, OpDecl, RuntimeOptions, error::JsError};
use std::sync::{Arc, Mutex};


#[derive(Debug, Clone)]
pub enum WorkflowStdout {
    Stdout(String),
    Stderr(String),
}

#[derive(Debug, Clone)]
pub struct OpStateWorkflowData {
    workflow_id: String,
    result: Vec<WorkflowStdout>,
    capture_stdout: bool,
}

impl OpStateWorkflowData {
    pub fn new(workflow_id: &str, capture_stdout: bool) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
            result: Vec::new(),
            capture_stdout,
        }
    }
    
    pub fn get_workflow_id(&self) -> &str {
        &self.workflow_id
    }
    
    pub fn add_result(&mut self, stdout: WorkflowStdout) {
        if self.capture_stdout {
            self.result.push(stdout);
        }
    }

}


/// Executes the given JavaScript code within a `JsRuntime` configured with custom operations.
///
/// # Overview
/// Runs the provided JavaScript `script` in a new `JsRuntime` instance, registering the supplied vector of `OpDecl` as custom operations (ops) via an extension. Use `op2` to define these operations.
///
/// # Arguments
/// - `script`: The JavaScript code to execute as a string.
/// - `ext`: A vector of `OpDecl` representing custom operations to be registered in the runtime.
///
/// # Returns
/// - `Ok(())`: If the script executes successfully.
/// - `Err(Box<JsError>)`: If an error occurs during execution.
///
///
/// # Notes
/// - The extension is registered with the name "ext".
/// - The script is always executed as the module "workflow.js".
///
/// # Errors
/// - Any JavaScript execution error is returned as `Box<JsError>`.
#[allow(unused)]
pub(crate) fn run_script(script: &str, ext: Vec<OpDecl>, workflow_data: Option<Arc<Mutex<OpStateWorkflowData>>>) -> Result<(), Box<JsError>> {
    // Register the extension with the provided operations
    let extension = Extension {
        name: "ext",
        ops: std::borrow::Cow::Owned(ext),
        ..Default::default()
    };

    // Create a new JsRuntime with the extension
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![extension],
        ..Default::default()
    });
    
    if let Some(workflow_data) = workflow_data {
        runtime.op_state().borrow_mut().put(workflow_data);
    }


    // Execute the provided script in the runtime
    let result = runtime.execute_script("workflow.js", script.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use deno_core::{op2, OpState};

    #[test]
    fn test_extension() {
        #[op2]
        fn test_op(#[serde] a: Vec<i32>) -> i32 {
            a.iter().sum()
        }

        let script = r#"
        console.log("Hello World! From Sapphillon Runtime! with JavaScript and Deno!");
        console.log("Sum of [1, 2, 3, 4, 5]", Deno.core.ops.test_op([1, 2, 3, 4, 5]));
        "#;

        let result = run_script(script, vec![test_op()], None);
        println!("[test_extension] result: {result:?}");
    }

    #[test]
    fn test_run_script() {
        let script = "1 + 1;";

        let result = run_script(script, vec![], None);
        assert!(result.is_ok(), "Script should run successfully");
    }
    #[test]
    fn test_run_script_hello() {
        let script = "a = 1 + 1; console.log('Hello, world!');console.log(a);";

        let result = run_script(script, vec![], None);
        assert!(result.is_ok(), "Script should run successfully");
    }

    #[test]
    fn test_run_script_opstate_workflow_data() {

        // テスト用op: opstateからworkflow_idを取得
        #[op2]
        #[string]
        fn get_workflow_id(state: &mut OpState) -> String {
            let data = state.borrow::<Arc<Mutex<OpStateWorkflowData>>>().lock().unwrap();
            data.workflow_id.clone()
        }
        use std::sync::{Arc, Mutex};

        // テスト用workflow_dataを生成
        let workflow_data = OpStateWorkflowData {
            workflow_id: "test_id_123".to_string(),
            result: vec![],
            capture_stdout: false,
        };
        let workflow_data_arc = Arc::new(Mutex::new(workflow_data.clone()));

        // JSスクリプトでopを呼び出し
        let script = r#"
            let id = Deno.core.ops.get_workflow_id();
            console.log("Workflow ID:", id);
            if (id !== "test_id_123") {
                throw new Error("workflow_id not injected into opstate!");
            }
        "#;

        let result = run_script(
            script,
            vec![get_workflow_id()],
            Some(workflow_data_arc),
        );
        assert!(result.is_ok(), "workflow_id should be accessible from opstate");
    }
}
