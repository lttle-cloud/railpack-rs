use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;

#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Option<LogLevel> {
        match s {
            "info" => Some(LogLevel::Info),
            "warn" => Some(LogLevel::Warn),
            "error" => Some(LogLevel::Error),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: Option<LogLevel>,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub key: String,
    pub value_json: String,
}

#[derive(Debug, Clone)]
pub struct BuildResult {
    pub success: bool,
    pub railpack_version: String,
    pub serialized_plan: Option<String>,
    pub detected_providers: Vec<String>,
    pub resolved_packages: Vec<(String, String)>,
    pub metadata: Vec<Metadata>,
    pub logs: Vec<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

pub type Command = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Layer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<Command>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caches: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deploy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<Layer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Layer>>,
    #[serde(rename = "startCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caches: Option<HashMap<String, Cache>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<Deploy>,
}

impl BuildResult {
    pub fn plan(&self) -> Option<BuildPlan> {
        self.serialized_plan
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok())
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub directory: String,
    pub env_vars: Vec<String>,
    pub verbose: bool,
    pub railpack_version: String,
    pub build_command: String,
    pub start_command: String,
    pub config_file_path: String,
    pub error_missing_start_command: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: String::new(),
            env_vars: Vec::new(),
            verbose: false,
            railpack_version: String::from("dev"),
            build_command: String::new(),
            start_command: String::new(),
            config_file_path: String::new(),
            error_missing_start_command: false,
        }
    }
}

impl Config {
    pub fn new(directory: impl Into<String>) -> Self {
        Self {
            directory: directory.into(),
            ..Default::default()
        }
    }

    pub fn with_env_vars(mut self, env_vars: Vec<String>) -> Self {
        self.env_vars = env_vars;
        self
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_railpack_version(mut self, version: impl Into<String>) -> Self {
        self.railpack_version = version.into();
        self
    }

    pub fn with_build_command(mut self, command: impl Into<String>) -> Self {
        self.build_command = command.into();
        self
    }

    pub fn with_start_command(mut self, command: impl Into<String>) -> Self {
        self.start_command = command.into();
        self
    }
}

unsafe fn c_str_to_string(ptr: *const i8) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

unsafe fn c_str_array_to_vec(ptr: *mut *const i8, count: usize) -> Vec<String> {
    if ptr.is_null() || count == 0 {
        return Vec::new();
    }

    let slice = std::slice::from_raw_parts(ptr, count);
    slice.iter().map(|&p| c_str_to_string(p)).collect()
}

pub fn generate_build_plan(config: &Config) -> Result<BuildResult, String> {
    let directory = CString::new(config.directory.as_str())
        .map_err(|e| format!("Invalid directory path: {}", e))?;

    let railpack_version = CString::new(config.railpack_version.as_str())
        .map_err(|e| format!("Invalid railpack version: {}", e))?;

    let build_command = CString::new(config.build_command.as_str())
        .map_err(|e| format!("Invalid build command: {}", e))?;

    let start_command = CString::new(config.start_command.as_str())
        .map_err(|e| format!("Invalid start command: {}", e))?;

    let config_file_path = CString::new(config.config_file_path.as_str())
        .map_err(|e| format!("Invalid config file path: {}", e))?;

    let env_vars: Vec<CString> = config
        .env_vars
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap())
        .collect();

    let mut env_vars_ptrs: Vec<*const i8> = env_vars.iter().map(|s| s.as_ptr()).collect();

    let mut c_config = RpConfig {
        directory: directory.as_ptr(),
        env_vars: env_vars_ptrs.as_mut_ptr(),
        env_count: env_vars_ptrs.len(),
        verbose: config.verbose,
        railpack_version: railpack_version.as_ptr(),
        build_command: build_command.as_ptr(),
        start_command: start_command.as_ptr(),
        config_file_path: config_file_path.as_ptr(),
        error_missing_start_command: config.error_missing_start_command,
    };

    let result_ptr = unsafe { rp_generate_build_plan(&mut c_config as *mut RpConfig) };

    if result_ptr.is_null() {
        return Err("Failed to generate build plan: null result".to_string());
    }

    let result = unsafe { &*result_ptr };

    let build_result = BuildResult {
        success: result.success,
        railpack_version: unsafe { c_str_to_string(result.railpack_version) },
        serialized_plan: unsafe {
            let plan_str = c_str_to_string(result.serialized_plan);
            if plan_str.is_empty() {
                None
            } else {
                Some(plan_str)
            }
        },
        detected_providers: unsafe {
            c_str_array_to_vec(
                result.detected_providers as *mut *const i8,
                result.detected_providers_count,
            )
        },
        resolved_packages: unsafe {
            if result.resolved_packages.is_null() || result.resolved_packages_count == 0 {
                Vec::new()
            } else {
                let slice = std::slice::from_raw_parts(
                    result.resolved_packages,
                    result.resolved_packages_count,
                );
                slice
                    .iter()
                    .map(|kv| (c_str_to_string(kv.key), c_str_to_string(kv.value)))
                    .collect()
            }
        },
        metadata: unsafe {
            if result.metadata.is_null() || result.metadata_count == 0 {
                Vec::new()
            } else {
                let slice = std::slice::from_raw_parts(result.metadata, result.metadata_count);
                slice
                    .iter()
                    .map(|m| Metadata {
                        key: c_str_to_string(m.key),
                        value_json: c_str_to_string(m.value_json),
                    })
                    .collect()
            }
        },
        logs: unsafe {
            if result.logs.is_null() || result.logs_count == 0 {
                Vec::new()
            } else {
                let slice = std::slice::from_raw_parts(result.logs, result.logs_count);
                slice
                    .iter()
                    .map(|log| LogEntry {
                        level: LogLevel::from_str(c_str_to_string(log.level).as_str()),
                        message: c_str_to_string(log.msg),
                    })
                    .collect()
            }
        },
    };

    unsafe {
        if !result.detected_providers.is_null() && result.detected_providers_count > 0 {
            let providers_slice = std::slice::from_raw_parts(
                result.detected_providers,
                result.detected_providers_count,
            );
            for &provider in providers_slice {
                rp_mem_free(provider as *mut std::ffi::c_void);
            }
            rp_mem_free(result.detected_providers as *mut std::ffi::c_void);
        }

        if !result.resolved_packages.is_null() && result.resolved_packages_count > 0 {
            let packages_slice = std::slice::from_raw_parts(
                result.resolved_packages,
                result.resolved_packages_count,
            );
            for kv in packages_slice {
                rp_mem_free(kv.key as *mut std::ffi::c_void);
                rp_mem_free(kv.value as *mut std::ffi::c_void);
            }
            rp_mem_free(result.resolved_packages as *mut std::ffi::c_void);
        }

        if !result.metadata.is_null() && result.metadata_count > 0 {
            let meta_slice = std::slice::from_raw_parts(result.metadata, result.metadata_count);
            for m in meta_slice {
                rp_mem_free(m.key as *mut std::ffi::c_void);
                rp_mem_free(m.value_json as *mut std::ffi::c_void);
            }
            rp_mem_free(result.metadata as *mut std::ffi::c_void);
        }

        if !result.logs.is_null() && result.logs_count > 0 {
            let logs_slice = std::slice::from_raw_parts(result.logs, result.logs_count);
            for log in logs_slice {
                rp_mem_free(log.level as *mut std::ffi::c_void);
                rp_mem_free(log.msg as *mut std::ffi::c_void);
            }
            rp_mem_free(result.logs as *mut std::ffi::c_void);
        }

        if !result.railpack_version.is_null() {
            rp_mem_free(result.railpack_version as *mut std::ffi::c_void);
        }

        if !result.serialized_plan.is_null() {
            rp_mem_free(result.serialized_plan as *mut std::ffi::c_void);
        }

        rp_mem_free(result_ptr as *mut std::ffi::c_void);
    }

    Ok(build_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = Config::new("/test/directory")
            .with_verbose(true)
            .with_railpack_version("1.0.0")
            .with_build_command("npm run build")
            .with_start_command("npm start");

        assert_eq!(config.directory, "/test/directory");
        assert!(config.verbose);
        assert_eq!(config.railpack_version, "1.0.0");
        assert_eq!(config.build_command, "npm run build");
        assert_eq!(config.start_command, "npm start");
    }
}
