use std::io::Write;

use anyhow::{Context, Result};

/// Output format specifies the formating which will be used when writing to output
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    List,
}

impl OutputFormat {
    /// Writes prefixes to writer using specified output format
    pub fn write<W: Write>(&self, prefixes: Vec<String>, w: W) -> Result<()> {
        match self {
            OutputFormat::Json => self.write_json(prefixes, w),
            OutputFormat::Yaml => self.write_yaml(prefixes, w),
            OutputFormat::List => self.write_list(prefixes, w),
        }
    }

    fn write_json<W: Write>(&self, prefixes: Vec<String>, mut w: W) -> Result<()> {
        let json = serde_json::to_string(&prefixes)
            .with_context(|| "failed to serialize prefixes to JSON")?;
        writeln!(w, "{json}").with_context(|| "failed to write output")?;

        Ok(())
    }

    fn write_yaml<W: Write>(&self, prefixes: Vec<String>, mut w: W) -> Result<()> {
        let yaml = serde_yaml::to_string(&prefixes)
            .with_context(|| "failed to serialize prefixes to YAML")?;
        writeln!(w, "{yaml}").with_context(|| "failed to write output")?;

        Ok(())
    }

    fn write_list<W: Write>(&self, prefixes: Vec<String>, mut w: W) -> Result<()> {
        for prefix in prefixes {
            writeln!(w, "{prefix}")?;
        }

        Ok(())
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "list" => Ok(OutputFormat::List),
            _ => Err(anyhow::anyhow!("Unknown output format: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid_cases() {
        // Test lowercase
        assert!(matches!(
            OutputFormat::from_str("json").unwrap(),
            OutputFormat::Json
        ));
        assert!(matches!(
            OutputFormat::from_str("yaml").unwrap(),
            OutputFormat::Yaml
        ));
        assert!(matches!(
            OutputFormat::from_str("list").unwrap(),
            OutputFormat::List
        ));

        // Test uppercase
        assert!(matches!(
            OutputFormat::from_str("JSON").unwrap(),
            OutputFormat::Json
        ));
        assert!(matches!(
            OutputFormat::from_str("YAML").unwrap(),
            OutputFormat::Yaml
        ));
        assert!(matches!(
            OutputFormat::from_str("LIST").unwrap(),
            OutputFormat::List
        ));

        // Test mixed case
        assert!(matches!(
            OutputFormat::from_str("Json").unwrap(),
            OutputFormat::Json
        ));
        assert!(matches!(
            OutputFormat::from_str("YaMl").unwrap(),
            OutputFormat::Yaml
        ));
        assert!(matches!(
            OutputFormat::from_str("LiSt").unwrap(),
            OutputFormat::List
        ));
    }

    #[test]
    fn test_from_str_invalid_cases() {
        let result = OutputFormat::from_str("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Unknown output format: ");

        let result = OutputFormat::from_str("xml");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Unknown output format: xml"
        );
    }

    #[test]
    fn test_write_json() {
        let format = OutputFormat::Json;
        let prefixes = vec![
            "2001:678:1e0::/56".to_string(),
            "2001:678:1e0:100::/56".to_string(),
        ];
        let mut buffer = Vec::new();

        let result = format.write(prefixes, &mut buffer);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(
            output.trim(),
            r#"["2001:678:1e0::/56","2001:678:1e0:100::/56"]"#
        );
    }

    #[test]
    fn test_write_json_empty_vec() {
        let format = OutputFormat::Json;
        let prefixes = vec![];
        let mut buffer = Vec::new();

        let result = format.write(prefixes, &mut buffer);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output.trim(), "[]");
    }

    #[test]
    fn test_write_yaml() {
        let format = OutputFormat::Yaml;
        let prefixes = vec![
            "2001:678:1e0::/56".to_string(),
            "2001:678:1e0:100::/56".to_string(),
        ];
        let mut buffer = Vec::new();

        let result = format.write(prefixes, &mut buffer);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(
            output.trim(),
            "- 2001:678:1e0::/56\n- 2001:678:1e0:100::/56"
        );
    }

    #[test]
    fn test_write_yaml_empty_vec() {
        let format = OutputFormat::Yaml;
        let prefixes = vec![];
        let mut buffer = Vec::new();

        let result = format.write(prefixes, &mut buffer);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output.trim(), "[]");
    }

    #[test]
    fn test_write_list() {
        let format = OutputFormat::List;
        let prefixes = vec![
            "2001:678:1e0::/56".to_string(),
            "2001:678:1e0:100::/128".to_string(),
            "192.168.178.0/24".to_string(),
        ];
        let mut buffer = Vec::new();

        let result = format.write(prefixes, &mut buffer);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "2001:678:1e0::/56");
        assert_eq!(lines[1], "2001:678:1e0:100::/128");
        assert_eq!(lines[2], "192.168.178.0/24");
    }

    #[test]
    fn test_write_list_empty_vec() {
        let format = OutputFormat::List;
        let prefixes = vec![];
        let mut buffer = Vec::new();

        let result = format.write(prefixes, &mut buffer);
        assert!(result.is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "");
    }
}
