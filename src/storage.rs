// Ralqlator - A command line calculator
//
// Copyright (C) 2026 Ralqlator Contributors
//
// This program is free software: you can redistribute it and/or modify
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

//! Storage module for persisting user-defined functions, sequences, and constants
//!
//! Saves user definitions to ~/.ralqlator file in TOML format.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::functions::UserFunctions;
use crate::repl::UserConstants;

/// Storage operation result
pub type StorageResult<T> = Result<T, String>;

/// Default storage file name (hidden file in home directory)
const STORAGE_FILE_NAME: &str = ".ralqlator";

/// Data structure for serialization
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct UserData {
    #[serde(default)]
    pub functions: HashMap<String, FunctionData>,
    #[serde(default)]
    pub constants: HashMap<String, f64>,
}

/// Function data structure for serialization
#[derive(Serialize, Deserialize, Clone)]
pub struct FunctionData {
    pub params: Vec<String>,
    pub expr: String,
}

/// Get the storage file path (~/.ralqlator)
pub fn get_storage_path() -> Result<PathBuf, String> {
    let home = std::env::var("HOME")
        .map_err(|_| "HOME environment variable not set".to_string())?;
    Ok(PathBuf::from(home).join(STORAGE_FILE_NAME))
}

/// Save user functions and constants to storage file
pub fn save_user_data(
    user_functions: &UserFunctions,
    user_constants: &UserConstants,
) -> Result<(), String> {
    let storage_path = get_storage_path()?;

    // Build user data structure
    let mut user_data = UserData::default();

    // Extract functions
    let funcs = user_functions.lock().unwrap();
    for (name, (params, expr)) in funcs.iter() {
        user_data.functions.insert(
            name.clone(),
            FunctionData {
                params: params.clone(),
                expr: expr.clone(),
            },
        );
    }
    drop(funcs);

    // Extract constants
    let consts = user_constants.lock().unwrap();
    for (name, value) in consts.iter() {
        user_data.constants.insert(name.clone(), *value);
    }
    drop(consts);

    // Serialize to TOML
    let toml_str = toml::to_string_pretty(&user_data)
        .map_err(|e| format!("Failed to serialize user data: {}", e))?;

    // Write to file
    fs::write(&storage_path, toml_str)
        .map_err(|e| format!("Failed to write to {}: {}", storage_path.display(), e))?;

    Ok(())
}

/// Load user functions and constants from storage file
pub fn load_user_data(
    user_functions: &UserFunctions,
    user_constants: &UserConstants,
) -> Result<usize, String> {
    let storage_path = get_storage_path()?;

    // Check if file exists
    if !storage_path.exists() {
        return Ok(0);
    }

    // Read file content
    let content = fs::read_to_string(&storage_path)
        .map_err(|e| format!("Failed to read {}: {}", storage_path.display(), e))?;

    // Parse TOML
    let user_data: UserData = toml::from_str(&content)
        .map_err(|e| format!("Failed to parse TOML: {}", e))?;

    // Load functions
    let mut funcs = user_functions.lock().unwrap();
    for (name, func_data) in user_data.functions {
        funcs.insert(name, (func_data.params, func_data.expr));
    }
    let func_count = funcs.len();
    drop(funcs);

    // Load constants
    let mut consts = user_constants.lock().unwrap();
    for (name, value) in user_data.constants {
        consts.insert(name, value);
    }
    let const_count = consts.len();
    drop(consts);

    Ok(func_count + const_count)
}

/// Delete any user definition (function, sequence, or constant)
/// Supports case-insensitive matching with helpful error messages
pub fn delete_user_definition(
    name: &str,
    user_functions: &UserFunctions,
    user_constants: &UserConstants,
) -> StorageResult<bool> {
    // Try exact match first (case-sensitive)
    {
        let mut funcs = user_functions.lock().unwrap();
        if funcs.remove(name).is_some() {
            drop(funcs);
            save_user_data(user_functions, user_constants)?;
            return Ok(true);
        }
    }

    {
        let mut consts = user_constants.lock().unwrap();
        if consts.remove(name).is_some() {
            drop(consts);
            save_user_data(user_functions, user_constants)?;
            return Ok(true);
        }
    }

    // If exact match fails, try case-insensitive search to provide helpful message
    let name_lower = name.to_lowercase();
    
    // Check functions for case-insensitive match
    {
        let funcs = user_functions.lock().unwrap();
        for (key, _) in funcs.iter() {
            if key.to_lowercase() == name_lower {
                return Err(format!(
                    "Definition '{}' not found. Did you mean '{}'? (names are case-sensitive)",
                    name, key
                ));
            }
        }
    }

    // Check constants for case-insensitive match
    {
        let consts = user_constants.lock().unwrap();
        for (key, _) in consts.iter() {
            if key.to_lowercase() == name_lower {
                return Err(format!(
                    "Definition '{}' not found. Did you mean '{}'? (names are case-sensitive)",
                    name, key
                ));
            }
        }
    }

    Ok(false)
}

/// Get storage file path for display
pub fn get_storage_path_string() -> String {
    get_storage_path()
        .unwrap_or_else(|_| PathBuf::from(STORAGE_FILE_NAME))
        .display()
        .to_string()
}
