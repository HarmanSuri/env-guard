pub mod entropy;

use self::entropy::scan_line_entropy;
use crate::git::StagedFile;

#[derive(Debug, PartialEq, Eq)]
pub struct FoundSecret {
    pub file_path: String,
    pub line_number: usize,
    pub detector: String,
    pub secret: String,
    pub line_content: String,
}

#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub min_entropy_length: usize,
    pub entropy_threshold: f64,
    pub enable_entropy: bool,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            min_entropy_length: 16,
            entropy_threshold: 4.0,
            enable_entropy: true,
        }
    }
}

pub struct Scanner {
    config: ScannerConfig,
}

impl Scanner {
    pub fn new(config: ScannerConfig) -> Self {
        Self { config }
    }

    pub fn scan(&self, staged_files: &[StagedFile]) -> Vec<FoundSecret> {
        let mut findings = Vec::new();

        for file in staged_files {
            for line in &file.lines {
                if self.config.enable_entropy {
                    let results = scan_line_entropy(
                        &line.text,
                        self.config.min_entropy_length,
                        self.config.entropy_threshold,
                    );

                    for res in results {
                        findings.push(FoundSecret {
                            file_path: file.path.clone(),
                            line_number: line.line_number,
                            detector: "High Entropy".to_string(),
                            secret: res.token.to_string(),
                            line_content: line.text.clone(),
                        });
                    }
                }
            }
        }

        findings
    }
}
