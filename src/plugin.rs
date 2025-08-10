use deno_core::OpDecl;
use std::borrow::Cow;
use crate::proto::sapphillon::v1::{PluginFunction, PluginPackage};

pub struct CorePluginFunction {
    pub id: String,
    pub name: String,
    pub func: Cow<'static, OpDecl>,
}

impl CorePluginFunction {
    pub fn new(id: String, name: String, func: OpDecl) -> Self {
        Self {
            id,
            name,
            func: Cow::Owned(func),
        }

    }

    pub fn new_from_plugin_function(plugin_function: &PluginFunction, function: OpDecl) -> Self {
        Self {
            id: plugin_function.function_id.clone(),
            name: plugin_function.function_name.clone(),
            func: Cow::Owned(function),
        }
    }
}

pub struct CorePluginPackage {
    pub id: String,
    pub name: String,
    pub functions: Vec<CorePluginFunction>,
}

impl CorePluginPackage {
    pub fn new(id: String, name: String, functions: Vec<CorePluginFunction>) -> Self {
        Self {
            id,
            name,
            functions,
        }
    }

    pub fn new_from_plugin_package(plugin_package: &PluginPackage, functions: Vec<CorePluginFunction>) -> Self {

        Self {
            id: plugin_package.package_id.clone(),
            name: plugin_package.package_name.clone(),
            functions,
        }
    }
}