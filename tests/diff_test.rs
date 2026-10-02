use env_guard::git::{StagedFile, parse_unified_diff};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unified_diff() {
        let sample_diff = r#"
diff --git a/src/config.rs b/src/config.rs
index 83db48f..f735b23 100644
--- a/src/config.rs
+++ b/src/config.rs
@@ -10,0 +15,2 @@
+const API_KEY: &str = "sk_live_123456789";
+const PORT: u16 = 8080;
"#;

        let result: Vec<StagedFile> = parse_unified_diff(sample_diff);
        print!("{}", result.len());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, "src/config.rs");
        assert_eq!(result[0].lines.len(), 2);

        assert_eq!(result[0].lines[0].line_number, 15);
        assert_eq!(
            result[0].lines[0].text,
            "const API_KEY: &str = \"sk_live_123456789\";"
        );

        assert_eq!(result[0].lines[1].line_number, 16);
        assert_eq!(result[0].lines[1].text, "const PORT: u16 = 8080;");
    }
}
