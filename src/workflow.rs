use crate::proto::sapphillon;
use crate::plugin::CorePluginPackage;

pub enum WorkflowResult {
    NotExecuted,
    Running,
    Success(String),
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