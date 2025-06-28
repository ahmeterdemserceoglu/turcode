pub mod commands;

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Ignored,
    Conflicted,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusEntry {
    pub path: String,
    pub status: FileStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub remote: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
}

// Check if a directory is a Git repository
pub fn is_git_repo(path: &Path) -> bool {
    let git_dir = path.join(".git");
    git_dir.exists() && git_dir.is_dir()
}

// Get current Git status (mock implementation)
pub fn get_repo_status(path: &Path) -> Result<Vec<StatusEntry>, String> {
    // This would normally use libgit2 or subprocess to git
    // For now, just return mock data
    
    if !is_git_repo(path) {
        return Err("Not a Git repository".to_string());
    }
    
    let mock_status = vec![
        StatusEntry {
            path: "src/main.rs".to_string(),
            status: FileStatus::Modified,
        },
        StatusEntry {
            path: "docs/README.md".to_string(),
            status: FileStatus::Modified,
        },
        StatusEntry {
            path: "new_file.txt".to_string(),
            status: FileStatus::Untracked,
        },
    ];
    
    Ok(mock_status)
}

// Get list of branches (mock implementation)
pub fn get_branches(path: &Path) -> Result<Vec<BranchInfo>, String> {
    // This would normally use libgit2 or subprocess to git
    // For now, just return mock data
    
    if !is_git_repo(path) {
        return Err("Not a Git repository".to_string());
    }
    
    let mock_branches = vec![
        BranchInfo {
            name: "main".to_string(),
            is_current: true,
            is_remote: false,
            remote: None,
        },
        BranchInfo {
            name: "develop".to_string(),
            is_current: false,
            is_remote: false,
            remote: None,
        },
        BranchInfo {
            name: "origin/main".to_string(),
            is_current: false,
            is_remote: true,
            remote: Some("origin".to_string()),
        },
    ];
    
    Ok(mock_branches)
}

// Get recent commits (mock implementation)
pub fn get_commits(path: &Path, limit: usize) -> Result<Vec<CommitInfo>, String> {
    // This would normally use libgit2 or subprocess to git
    // For now, just return mock data
    
    if !is_git_repo(path) {
        return Err("Not a Git repository".to_string());
    }
    
    let mock_commits = vec![
        CommitInfo {
            hash: "abcdef1234567890abcdef1234567890abcdef12".to_string(),
            short_hash: "abcdef1".to_string(),
            author: "John Doe <john@example.com>".to_string(),
            date: "2025-06-28T12:34:56Z".to_string(),
            message: "Fix bug in file processing".to_string(),
        },
        CommitInfo {
            hash: "1234567890abcdef1234567890abcdef12345678".to_string(),
            short_hash: "1234567".to_string(),
            author: "Jane Smith <jane@example.com>".to_string(),
            date: "2025-06-27T15:45:23Z".to_string(),
            message: "Add new feature".to_string(),
        },
    ];
    
    // Limit the number of commits returned
    let result = mock_commits.into_iter().take(limit).collect();
    
    Ok(result)
} 