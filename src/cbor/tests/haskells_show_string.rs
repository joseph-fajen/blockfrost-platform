use super::{generate_cases, CaseType};

#[test]
fn proptest_data_text_1000_large() {
    let cases = generate_cases(CaseType::DataText, 1000, 1000, None).unwrap();

    for case in cases.test_cases {
        let json_encoding: String = serde_json::to_string(&case.json).unwrap();
        let normal_string: String = serde_json::from_value(case.json).unwrap();
        let rust_emulation = haskell_show_string(&normal_string);
        assert_eq!(
            case.haskell_repr, rust_emulation,
            "\n\n   JSON encoding: {}\n    Haskell Show: {}\n  Rust emulation: {}\n",
            json_encoding, case.haskell_repr, rust_emulation
        );
    }
}

/// Haskell’s `Data.Text` (and `Data.String`) have "slightly" non-standard way
/// of displaying non-printable Unicode characters in their `Show` instances.
/// Let’s make sure we replicate that correctly in Rust.
fn haskell_show_string(s: &str) -> String {
    fn is_oct_digit(c: char) -> bool {
        c >= '0' && c <= '7'
    }

    let mut result = String::new();
    result.push('"');

    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\x07' => result.push_str("\\a"),            // Bell
            '\x08' => result.push_str("\\b"),            // Backspace
            '\x0C' => result.push_str("\\f"),            // Form Feed
            '\x0A' => result.push_str("\\n"),            // Line Feed
            '\x0D' => result.push_str("\\r"),            // Carriage Return
            '\x09' => result.push_str("\\t"),            // Horizontal Tab
            '\x0B' => result.push_str("\\v"),            // Vertical Tab
            c if c >= ' ' && c <= '~' => result.push(c), // Printable ASCII
            c => {
                let abbreviation = match c {
                    '\x00' => "NUL",
                    '\x01' => "SOH",
                    '\x02' => "STX",
                    '\x03' => "ETX",
                    '\x04' => "EOT",
                    '\x05' => "ENQ",
                    '\x06' => "ACK",
                    '\x0E' => "SO",
                    '\x0F' => "SI",
                    '\x10' => "DLE",
                    '\x11' => "DC1",
                    '\x12' => "DC2",
                    '\x13' => "DC3",
                    '\x14' => "DC4",
                    '\x15' => "NAK",
                    '\x16' => "SYN",
                    '\x17' => "ETB",
                    '\x18' => "CAN",
                    '\x19' => "EM",
                    '\x1A' => "SUB",
                    '\x1B' => "ESC",
                    '\x1C' => "FS",
                    '\x1D' => "GS",
                    '\x1E' => "RS",
                    '\x1F' => "US",
                    '\x7F' => "DEL",
                    _ => "",
                };
                if !abbreviation.is_empty() {
                    result.push('\\');
                    result.push_str(abbreviation);

                    // Insert \& if next character would be confusing
                    if let Some(&next_c) = chars.peek() {
                        if abbreviation == "SO" && next_c == 'H' {
                            result.push_str("\\&");
                        }
                    }
                } else if c <= '\x7F' {
                    // Control characters without abbreviation (use octal escape)
                    let code = c as u32 % 256;
                    let escape_seq = format!("{:o}", code);
                    result.push('\\');
                    result.push_str(&escape_seq);

                    // Insert \& if next character is an octal digit
                    if let Some(&next_c) = chars.peek() {
                        if is_oct_digit(next_c) {
                            result.push_str("\\&");
                        }
                    }
                } else {
                    // Characters with code > 127 (use decimal escape)
                    let code = c as u32;
                    result.push('\\');
                    result.push_str(&code.to_string());

                    // Insert \& if next character is a decimal digit
                    if let Some(&next_c) = chars.peek() {
                        if next_c.is_ascii_digit() {
                            result.push_str("\\&");
                        }
                    }
                }
            }
        }
    }

    result.push('"');
    result
}
