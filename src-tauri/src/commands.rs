use serde::Serialize;
use tauri::{AppHandle, Manager};
use crate::kb::KbTemplate;

/// 返回全部知识库模板
#[tauri::command]
pub fn get_kb_templates() -> Vec<KbTemplate> {
    crate::kb::all_templates()
}

/// 复制文本到剪贴板
#[tauri::command]
pub fn copy_text(text: String) -> Result<(), String> {
    let mut cb = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    cb.set_text(text).map_err(|e| e.to_string())?;
    Ok(())
}