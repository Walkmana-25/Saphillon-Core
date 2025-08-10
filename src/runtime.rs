#![warn(clippy::field_reassign_with_default)]

use deno_core::{Extension, JsRuntime, OpDecl, RuntimeOptions, error::JsError};

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
pub(crate) fn run_script(script: &str, ext: Vec<OpDecl>) -> Result<(), Box<JsError>> {
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

    // Execute the provided script in the runtime
    runtime.execute_script("workflow.js", script.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use deno_core::op2;

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

        let result = run_script(script, vec![test_op()]);
        println!("[test_extension] result: {result:?}");
    }

    #[test]
    fn test_run_script() {
        let script = "1 + 1;";

        let result = run_script(script, vec![]);
        assert!(result.is_ok(), "Script should run successfully");
    }
    #[test]
    fn test_run_script_hello() {
        let script = "a = 1 + 1; console.log('Hello, world!');console.log(a);";

        let result = run_script(script, vec![]);
        assert!(result.is_ok(), "Script should run successfully");
    }
}
