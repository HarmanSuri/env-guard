pub mod entropy;
pub mod patterns;

use self::entropy::scan_line_entropy;
use self::patterns::{PatternMatcher, PatternRule};
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
    pub enable_patterns: bool,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            min_entropy_length: 16,
            entropy_threshold: 3.9,
            enable_entropy: true,
            enable_patterns: true,
        }
    }
}

pub struct Scanner {
    config: ScannerConfig,
    pattern_matcher: PatternMatcher,
}

impl Scanner {
    pub fn new(config: ScannerConfig) -> Result<Self, regex::Error> {
        let pattern_matcher = PatternMatcher::new(PatternRule::defaults())?;
        Ok(Self {
            config,
            pattern_matcher,
        })
    }

    pub fn scan(&self, staged_files: &[StagedFile]) -> Vec<FoundSecret> {
        let mut findings = Vec::new();

        for file in staged_files {
            for line in &file.lines {
                let mut matched_by_pattern = false;

                // Detect by pattern matching
                if self.config.enable_patterns {
                    let matcher_results = self.pattern_matcher.scan_line(&line.text);

                    if !matcher_results.is_empty() {
                        matched_by_pattern = true;
                        for res in matcher_results {
                            findings.push(FoundSecret {
                                file_path: file.path.clone(),
                                line_number: line.line_number,
                                detector: res.rule_name.to_string(),
                                secret: res.matched_secret.to_string(),
                                line_content: line.text.clone(),
                            });
                        }
                    }
                }

                // Fallback to detection by entropy
                if self.config.enable_entropy && !matched_by_pattern {
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
