use env_guard::git::{StagedFile, StagedLine};
use env_guard::scanner::{Scanner, ScannerConfig};

#[test]
fn test_scanner_dual_engine() {
    let mock_diff = vec![StagedFile {
        path: "src/config.rs".to_string(),
        lines: vec![
            // Line 1: Pattern Match (AWS Key) - Should be caught by Regex, skipping Entropy
            StagedLine {
                line_number: 12,
                text: "let aws_key = \"AKIAIOSFODNN7EXAMPLE\";".to_string(),
            },
            // Line 2: Generic Secret - Should be caught by Entropy fallback
            StagedLine {
                line_number: 13,
                text: "let secret = \"x9f3a21b8c0d4e7f9a1b2c3d4e5f6a7b\";".to_string(),
            },
            // Line 3: Safe Code - Should generate NO findings
            StagedLine {
                line_number: 14,
                text: "let config_path = \"/usr/local/bin/app\";".to_string(),
            },
        ],
    }];

    let scanner = Scanner::new(ScannerConfig::default()).unwrap();
    let findings = scanner.scan(&mock_diff);

    println!("{:#?}", findings);

    assert_eq!(findings.len(), 2);

    // Finding 1: Regex Pattern
    assert_eq!(findings[0].detector, "AWS Access Key ID");
    assert_eq!(findings[0].line_number, 12);
    assert_eq!(findings[0].secret, "AKIAIOSFODNN7EXAMPLE");

    // Finding 2: Entropy Fallback
    assert_eq!(findings[0].file_path, "src/config.rs");
    assert_eq!(findings[1].detector, "High Entropy");
    assert_eq!(findings[1].line_number, 13);
}
