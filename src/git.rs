use std::fmt;
use std::process::Command;

#[derive(Debug)]
pub enum GitError {
    GitNotInstalled,
    NotAGitRepository,
    CommandFailed(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::GitNotInstalled => write!(f, "Git binary was not found in system PATH."),
            GitError::NotAGitRepository => {
                write!(f, "Current directory is not a valid Git repository.")
            }
            GitError::CommandFailed(msg) => write!(f, "Git command failed: {}", msg),
        }
    }
}

impl std::error::Error for GitError {}

#[derive(Debug, PartialEq, Eq)]
pub struct StagedLine {
    pub line_number: usize,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StagedFile {
    pub path: String,
    pub lines: Vec<StagedLine>,
}

pub fn get_staged_changes() -> Result<Vec<StagedFile>, GitError> {
    let git_output = Command::new("git")
        .args(["diff", "--cached", "-U0"])
        .output()
        .map_err(|_| GitError::GitNotInstalled)?;

    if !git_output.status.success() {
        let stderr = String::from_utf8_lossy(&git_output.stderr);
        if stderr.contains("Not a git repository") {
            return Err(GitError::NotAGitRepository);
        }
        return Err(GitError::CommandFailed(stderr.trim().to_string()));
    }

    let diff_text = String::from_utf8_lossy(&git_output.stdout);
    return Ok(parse_unified_diff(&diff_text));
}

pub fn parse_unified_diff(diff_text: &str) -> Vec<StagedFile> {
    let mut files: Vec<StagedFile> = Vec::new();
    let mut current_file: Option<StagedFile> = None;
    let mut current_line_num: usize = 0;

    for line in diff_text.lines() {
        // 1. handle file header
        if line.starts_with("+++ b/") {
            // if there was previous processed file then push it to the list of files
            // take() clears current_file
            if let Some(file) = current_file.take() {
                if !file.lines.is_empty() {
                    files.push(file);
                }
            }

            // process newly found file path
            let path = line["+++ b/".len()..].to_string();
            current_file = Some(StagedFile {
                path,
                lines: Vec::new(),
            });
            continue;
        }

        // ignore delete or new file header
        if line.starts_with("+++ ") || line.starts_with("--- ") {
            continue;
        }

        // 2. detect hunk header for line counts
        if line.starts_with("@@ ") {
            if let Some(start_line) = parse_hunk_header(line) {
                current_line_num = start_line;
            }
            continue;
        }

        // 3. extract new lines
        if let Some(ref mut file) = current_file {
            if line.starts_with('+') {
                // ignore diff metadate headers like '+'
                let content = line[1..].to_string();
                file.lines.push(StagedLine {
                    line_number: current_line_num,
                    text: content,
                });
                current_line_num += 1;
            } else if !line.starts_with('-') {
                // ignore filer context lines, which  shoudn't happen with -U0
                current_line_num += 1;
            }
        }
    }

    // push final file in the loop
    if let Some(file) = current_file {
        if !file.lines.is_empty() {
            files.push(file);
        }
    }

    return files;
}

fn parse_hunk_header(hunk_line: &str) -> Option<usize> {
    let parts: Vec<&str> = hunk_line.split("@@").collect();
    if parts.len() < 2 {
        return None;
    }

    // fidn new target range
    let target_range = parts[1].split_whitespace().find(|p| p.starts_with('+'))?;

    // extract starting line number
    let line_str = target_range[1..].split(',').next()?;
    return line_str.parse::<usize>().ok();
}
