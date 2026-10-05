/// Represents a high-entropy string candidate found on a staged line.
#[derive(Debug, PartialEq)]
pub struct EntropyResult<'a> {
    pub token: &'a str,
    pub entropy: f64,
}

/// Scans a single line for high-entropy tokens exceeding `threshold`.
pub fn scan_line_entropy<'a>(
    line: &'a str,
    min_length: usize,
    threshold: f64,
) -> Vec<EntropyResult<'a>> {
    extract_candidate_tokens(line, min_length)
        .into_iter()
        .map(|token| {
            let entropy = calculate_entropy(token);
            EntropyResult { token, entropy }
        })
        .filter(|result| result.entropy >= threshold)
        .collect()
}

/// Extracts candidate string slices from a line of code for entropy analysis.
pub fn extract_candidate_tokens(line: &str, min_length: usize) -> Vec<&str> {
    line.split(|c: char| {
        c.is_whitespace() || c == '"' || c == '\'' || c == '`' || c == '=' || c == ';' || c == ','
    })
    // Trim common enclosing brackets/syntax from token boundaries
    .map(|token| {
        token.trim_matches(|c: char| {
            c == '['
                || c == ']'
                || c == '<'
                || c == '>'
                || c == '('
                || c == ')'
                || c == '{'
                || c == '}'
        })
    })
    // Filter out short tokens and common non-secret URLs
    .filter(|token| {
        token.len() >= min_length && !token.starts_with("http://") && !token.starts_with("https://")
    })
    .collect()
}

/// Calculates the Shannon Entropy of a string slice in bits per character.
pub fn calculate_entropy(text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }

    let mut counts = [0usize; 256];
    let len = text.len() as f64;

    // Step 1: Count character byte frequencies
    for &byte in text.as_bytes() {
        counts[byte as usize] += 1;
    }

    // Step 2: Compute Shannon Entropy using the formula
    let mut entropy = 0.0;
    for &count in counts.iter() {
        if count > 0 {
            // TODO: How do we compute P(x) and update the running entropy total?
            let px: f64 = (count as f64) / len;
            entropy += px * px.log2();
        }
    }

    -entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_entropy_known_values() {
        // Uniform characters -> minimal entropy (0.0 bits)
        assert_eq!(calculate_entropy("aaaaaaaaaaaaa"), 0.0);

        // Standard low-entropy path
        let path_entropy = calculate_entropy("/usr/local/bin/config");
        assert!(
            path_entropy < 3.6,
            "Expected path entropy < 3.5, got {}",
            path_entropy
        );

        // High-entropy random token
        let secret_entropy = calculate_entropy("AKIAIOSFODNN7EXAMPLE1234");
        assert!(
            secret_entropy > 4.0,
            "Expected secret entropy > 4.0, got {}",
            secret_entropy
        );
    }

    #[test]
    fn test_scan_line_entropy_detection() {
        let line = "let api_key = \"AKIAIOSFODNN7EXAMPLE1234\"; // config line";
        let results = scan_line_entropy(line, 16, 4.0);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].token, "AKIAIOSFODNN7EXAMPLE1234");
        assert!(results[0].entropy >= 4.0);
    }
}
