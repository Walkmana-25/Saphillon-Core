use crate::plugin::CorePluginPackage;
use crate::proto::sapphillon;
use crate::proto::sapphillon::v1::{WorkflowResult, WorkflowResultType};
use crate::runtime::{run_script, OpStateWorkflowData};
use prost_types::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

pub struct CoreWorkflowCode {
    /// Unique ID of the workflow code
    pub id: String,
    /// Deno OpDecl (workflow code body)
    pub code: String,
    /// List of plugin packages used in the workflow
    pub plugin_packages: Vec<CorePluginPackage>,

    pub code_revision: i32,
    pub result: Vec<sapphillon::v1::WorkflowResult>,
}

impl CoreWorkflowCode {
    /// Creates a new CoreWorkflowCode from the given ID, name, code, and plugin packages.
    ///
    /// # Arguments
    /// * `id` - Unique ID of the workflow code
    /// * `code` - Deno OpDecl (workflow code body)
    /// * `plugin_packages` - List of plugin packages used in the workflow
    /// * `code_revision` - Revision number of the code
    pub fn new(
        id: String,
        code: String,
        plugin_packages: Vec<CorePluginPackage>,
        code_revision: i32,
    ) -> Self {
        Self {
            id,
            code,
            plugin_packages,
            code_revision,
            result: Vec::new(),
        }
    }

    /// Executes the workflow code and appends a WorkflowResult to the result list.
    ///
    /// This method collects all OpDecls from the associated plugin packages, executes the workflow code
    /// using these operations, and records the execution result. The result includes metadata such as
    /// execution time, revision, exit code, and result type (success or failure). The result is appended
    /// to the `result` field of the struct.
    ///
    /// # Execution Flow
    /// 1. Collect OpDecls from all plugin packages.
    /// 2. Generate execution metadata (ID, display name, timestamp, revision).
    /// 3. Execute the workflow code using `run_script`.
    /// 4. Construct a `WorkflowResult` based on the execution outcome.
    /// 5. Append the result to the `result` vector.
    ///
    /// # Side Effects
    /// - Modifies the `result` field by adding a new `WorkflowResult`.
    pub fn run(&mut self) {
        // Collect OpDecls from plugin packages
        let mut ops = Vec::new();
        for pkg in &self.plugin_packages {
            for func in &pkg.functions {
                ops.push(func.func.clone().into_owned());
            }
        }

        // Execute the workflow code and record the result
        let now = SystemTime::now();
        let epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let id = format!("{}-{}", self.id, epoch.as_nanos());
        let display_name = format!("Run {}", epoch.as_secs());
        let ran_at = Some(Timestamp {
            seconds: epoch.as_secs() as i64,
            nanos: epoch.subsec_nanos() as i32,
        });
        let workflow_result_revision = self
            .result
            .last()
            .map(|r| r.workflow_result_revision + 1)
            .unwrap_or(1);
        
        let opstate_workflow_data = OpStateWorkflowData::new(
            &self.id,
            true
        );
        let result = run_script(&self.code, ops, Some(Arc::new(Mutex::new(opstate_workflow_data))));

        let (description, result, result_type, exit_code) = match result {
            Ok(data) => (
                "Success".to_string(),
                data.lock().unwrap().stdout_to_string(),
                WorkflowResultType::SuccessUnspecified as i32,
                0,
            ),
            Err(e) => (
                format!("Error: {e}"),
                format!("{e}"),
                WorkflowResultType::Failure as i32,
                1,
            ),
        };

        let result_obj = WorkflowResult {
            id,
            display_name,
            description,
            result,
            ran_at,
            result_type,
            exit_code,
            workflow_result_revision,
        };
        self.result.push(result_obj);
    }

    /// Creates a CoreWorkflowCode from a proto WorkflowCode.
    ///
    /// # Arguments
    /// * `workflow_code` - WorkflowCode defined in proto
    /// * `plugin_packages` - List of plugin packages used in the workflow
    pub fn new_from_proto(
        workflow_code: &sapphillon::v1::WorkflowCode,
        plugin_packages: Vec<CorePluginPackage>,
    ) -> Self {
        Self {
            id: workflow_code.id.clone(),
            code: workflow_code.code.clone(),
            plugin_packages,
            code_revision: workflow_code.code_revision,
            result: Vec::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::{CorePluginFunction, CorePluginPackage};
    use crate::proto::sapphillon::v1::WorkflowCode;

    // Generate a dummy CorePluginFunction for testing
    fn dummy_plugin_function() -> CorePluginFunction {
        // Use a dummy OpDecl that returns u32, same as in plugin.rs tests
        use deno_core::op2;
        #[op2(fast)]
        fn dummy_op() -> u32 {
            42
        }
        CorePluginFunction::new(
            "fid".to_string(),
            "fname".to_string(),
            "desc".to_string(),
            dummy_op(),
        )
    }

    // Generate a dummy CorePluginPackage for testing
    fn dummy_plugin_package() -> CorePluginPackage {
        CorePluginPackage::new(
            "pid".to_string(),
            "pname".to_string(),
            vec![dummy_plugin_function()],
        )
    }

    #[test]
    fn test_core_workflow_code_run_success() {
        let pkg = dummy_plugin_package();
        let mut code = CoreWorkflowCode::new("wid".to_string(), "console.log(1 + 1);".to_string(), vec![pkg], 1);
        code.run();
        assert_eq!(code.result.len(), 1);
        let res = &code.result[0];
        assert_eq!(res.exit_code, 0);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::SuccessUnspecified as i32
        );
        assert_eq!(res.result, "2\n");
    }

    #[test]
    fn test_core_workflow_code_run_failure() {
        let pkg = dummy_plugin_package();
        let mut code = CoreWorkflowCode::new(
            "wid".to_string(),
            "throw new Error('fail');".to_string(),
            vec![pkg],
            1,
        );
        code.run();
        assert_eq!(code.result.len(), 1);
        let res = &code.result[0];
        assert_eq!(res.exit_code, 1);
        assert_eq!(
            res.result_type,
            sapphillon::v1::WorkflowResultType::Failure as i32
        );
        assert!(res.result.contains("fail"));
    }
    // Generate a dummy WorkflowCode (proto) for testing
    fn dummy_proto_workflow_code() -> WorkflowCode {
        WorkflowCode {
            id: "wid".to_string(),
            code: "console.log('hello');".to_string(),
            code_revision: 1,
            ..Default::default()
        }
    }

    #[test]
    fn test_core_workflow_code_new() {
        let pkg = dummy_plugin_package();
        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "console.log('test');".to_string(),
            vec![pkg],
            2,
        );
        assert_eq!(code.id, "wid");
        assert_eq!(code.code, "console.log('test');");
        assert_eq!(code.plugin_packages.len(), 1);
        assert_eq!(code.code_revision, 2);
        assert!(code.result.is_empty());
    }

    #[test]
    fn test_core_workflow_code_new_from_proto() {
        let proto = dummy_proto_workflow_code();
        let pkg = dummy_plugin_package();
        let code = CoreWorkflowCode::new_from_proto(&proto, vec![pkg]);
        assert_eq!(code.id, proto.id);
        assert_eq!(code.code, proto.code);
        assert_eq!(code.plugin_packages.len(), 1);
        assert_eq!(code.code_revision, proto.code_revision);
        assert!(code.result.is_empty());
    }

    #[test]
    fn test_workflow_result_initial_state() {
        let pkg = dummy_plugin_package();
        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "console.log('test');".to_string(),
            vec![pkg],
            1,
        );
        assert!(code.result.is_empty(), "Initial results should be empty");
    }
}
