use tauri::command;
use std::path::{Path, PathBuf};
use crate::git::{StatusEntry, BranchInfo, CommitInfo, get_repo_status, get_branches, get_commits};

#[command]
pub async fn get_status(path: String) -> Result<Vec<StatusEntry>, String> {
    get_repo_status(Path::new(&path))
}

#[command]
pub async fn get_all_branches(path: String) -> Result<Vec<BranchInfo>, String> {
    get_branches(Path::new(&path))
}

#[command]
pub async fn get_recent_commits(path: String, limit: usize) -> Result<Vec<CommitInfo>, String> {
    get_commits(Path::new(&path), limit)
}

#[command]
pub async fn checkout_branch(path: String, branch_name: String) -> Result<(), String> {
    // Mock implementation
    // In a real application, this would use libgit2 to perform the operation
    Ok(())
}

#[command]
pub async fn create_branch(path: String, branch_name: String) -> Result<(), String> {
    // Mock implementation
    // In a real application, this would use libgit2 to perform the operation
    Ok(())
}

#[command]
pub async fn commit(path: String, message: String) -> Result<CommitInfo, String> {
    // Mock implementation
    // In a real application, this would use libgit2 to perform the operation
    
    // Return mock data for now
    Ok(CommitInfo {
        hash: "0000000000000000000000000000000000000000".to_string(),
        short_hash: "0000000".to_string(),
        author: "Current User <user@example.com>".to_string(),
        date: chrono::Utc::now().to_rfc3339(),
        message,
    })
}

#[command]
pub async fn pull(path: String, remote: String, branch: String) -> Result<(), String> {
    // Mock implementation
    // In a real application, this would use libgit2 to perform the operation
    Ok(())
}

#[command]
pub async fn push(path: String, remote: String, branch: String) -> Result<(), String> {
    // Mock implementation
    // In a real application, this would use libgit2 to perform the operation
    Ok(())
}

#[command]
pub async fn stage_file(path: String, file_path: String) -> Result<(), String> {
    // Mock implementation
    // In a real application, this would use libgit2 to perform the operation
    Ok(())
}

#[command]
pub async fn unstage_file(path: String, file_path: String) -> Result<(), String> {
    // Mock implementation
    // In a real application, this would use libgit2 to perform the operation
    Ok(())
} 