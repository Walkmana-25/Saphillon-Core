use crate::proto::sapphillon;
use crate::plugin::CorePluginPackage;

pub enum WorkflowResult {
    /// Represents the workflow has not been executed yet
    NotExecuted,
    /// Represents the workflow is currently running 
    Running,
    /// Represents the workflow has completed successfully
    Success(String),
    /// Represents the workflow has failed with an error message
    Failure(String),
}

pub struct CoreWorkflowCode {
    /// Unique ID of the workflow code
    pub id: String,
    /// Deno OpDecl (workflow code body)
    pub code: String,
    /// List of plugin packages used in the workflow
    pub plugin_packages: Vec<CorePluginPackage>,
    
    pub code_revision: i32,
    pub result: WorkflowResult,
}

impl CoreWorkflowCode {
    /// Creates a new CoreWorkflowCode from the given ID, name, code, and plugin packages.
    ///
    /// # Arguments
    /// * `id` - Unique ID of the workflow code
    /// * `code` - Deno OpDecl (workflow code body)
    /// * `plugin_packages` - List of plugin packages used in the workflow
    /// * `code_revision` - Revision number of the code
    pub fn new(id: String, code: String, plugin_packages: Vec<CorePluginPackage>, code_revision: i32) -> Self {
        Self {
            id,
            code,
            plugin_packages,
            code_revision,
            result: WorkflowResult::NotExecuted,
        }
    }

    /// Creates a CoreWorkflowCode from a proto WorkflowCode.
    ///
    /// # Arguments
    /// * `workflow_code` - WorkflowCode defined in proto
    /// * `plugin_packages` - List of plugin packages used in the workflow
    pub fn new_from_proto(workflow_code: &sapphillon::v1::WorkflowCode, plugin_packages: Vec<CorePluginPackage>) -> Self {
        Self {
            id: workflow_code.id.clone(),
            code: workflow_code.code.clone(),
            plugin_packages,
            code_revision: workflow_code.code_revision,
            result: WorkflowResult::NotExecuted,
        }
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::{CorePluginPackage, CorePluginFunction};
    use crate::proto::sapphillon::v1::WorkflowCode;

    // ダミーCorePluginFunction生成
    fn dummy_plugin_function() -> CorePluginFunction {
        // OpDeclのダミーはplugin.rsのテスト同様にu32返却opで代用
        use deno_core::op2;
        #[op2(fast)]
        fn dummy_op() -> u32 { 42 }
        CorePluginFunction::new(
            "fid".to_string(),
            "fname".to_string(),
            "desc".to_string(),
            dummy_op()
        )
    }

    // ダミーCorePluginPackage生成
    fn dummy_plugin_package() -> CorePluginPackage {
        CorePluginPackage::new(
            "pid".to_string(),
            "pname".to_string(),
            vec![dummy_plugin_function()]
        )
    }

    // ダミーWorkflowCode(proto)生成
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
            2
        );
        assert_eq!(code.id, "wid");
        assert_eq!(code.code, "console.log('test');");
        assert_eq!(code.plugin_packages.len(), 1);
        assert_eq!(code.code_revision, 2);
        matches!(code.result, WorkflowResult::NotExecuted);
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
        matches!(code.result, WorkflowResult::NotExecuted);
    }

    #[test]
    fn test_workflow_result_initial_state() {
        let pkg = dummy_plugin_package();
        let code = CoreWorkflowCode::new(
            "wid".to_string(),
            "console.log('test');".to_string(),
            vec![pkg],
            1
        );
        if let WorkflowResult::NotExecuted = code.result {
            // OK
        } else {
            panic!("Initial WorkflowResult should be NotExecuted");
        }
    }
}