use regex::Regex;

/// Defines a rule for matching known secret signatures.
#[derive(Debug, Clone)]
pub struct PatternRule {
    pub name: String,
    pub pattern: &'static str,
}

impl PatternRule {
    /// Returns the default set of built-in secret signatures.
    pub fn defaults() -> Vec<PatternRule> {
        vec![
            PatternRule {
                name: "AWS Access Key ID".to_string(),
                pattern: r"\b(AKIA|ASIA)[0-9A-Z]{16}\b",
            },
            PatternRule {
                name: "GitHub Personal Access Token".to_string(),
                pattern: r"\bghp_[a-zA-Z0-9]{36}\b",
            },
            PatternRule {
                name: "Slack API Token".to_string(),
                pattern: r"xox[baprs]-[0-9a-zA-Z]{10,48}",
            },
            PatternRule {
                name: "Generic Private Key".to_string(),
                pattern: r"-----BEGIN[ A-Z0-9_-]+PRIVATE KEY-----",
            },
        ]
    }
}

/// Holds a compiled Regex alongside its metadata for high-speed scanning.
pub struct CompiledRule {
    pub name: String,
    pub regex: Regex,
}

pub struct PatternMatchResult<'a> {
    pub rule_name: &'a str,
    pub matched_secret: &'a str,
}

pub struct PatternMatcher {
    rules: Vec<CompiledRule>,
}

impl PatternMatcher {
    /// Constructs a matcher by compiling all provided pattern rules ONCE.
    pub fn new(rules: Vec<PatternRule>) -> Result<Self, regex::Error> {
        let mut compiled_rules = Vec::with_capacity(rules.len());

        for rule in rules {
            // compile rule.pattern into a Regex and push a CompiledRule into compiled_rules
            let re = Regex::new(rule.pattern)?;
            compiled_rules.push(CompiledRule {
                name: rule.name.to_string(),
                regex: re,
            });
        }

        Ok(Self {
            rules: compiled_rules,
        })
    }

    /// Scans a line against all compiled patterns.
    pub fn scan_line<'a>(&'a self, line: &'a str) -> Vec<PatternMatchResult<'a>> {
        let mut results = Vec::new();

        for rule in &self.rules {
            for mat in rule.regex.find_iter(line) {
                results.push(PatternMatchResult {
                    rule_name: &rule.name,
                    matched_secret: mat.as_str(),
                });
            }
        }

        results
    }
}
