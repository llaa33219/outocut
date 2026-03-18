use crate::models::*;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

pub fn parse_project(path: &Path) -> Result<Project> {
    let content = fs::read_to_string(path)?;
    let content = strip_comments(&content);
    let project: Project = serde_json::from_str(&content)?;
    Ok(project)
}

pub fn validate_project(path: &Path) -> Result<Project> {
    let project = parse_project(path)?;

    if project.settings.width == 0 || project.settings.height == 0 {
        return Err(anyhow!("Invalid dimensions: width and height must be > 0"));
    }

    if project.settings.fps == 0.0 {
        return Err(anyhow!("Invalid fps: must be > 0"));
    }

    if project.settings.duration <= 0.0 {
        return Err(anyhow!("Invalid duration: must be > 0"));
    }

    if let Some(main_comp) = project.compositions.get(&project.mainCompositionId) {
        if main_comp.duration != project.settings.duration {
            return Err(anyhow!(
                "Main composition duration must match project duration"
            ));
        }
    } else {
        return Err(anyhow!(
            "Main composition '{}' not found",
            project.mainCompositionId
        ));
    }

    Ok(project)
}

pub fn export_json(path: &Path, pretty: bool) -> Result<String> {
    let project = parse_project(path)?;
    if pretty {
        serde_json::to_string_pretty(&project)
    } else {
        serde_json::to_string(&project)
    }
    .map_err(|e| anyhow!(e))
}

fn strip_comments(json: &str) -> String {
    let mut result = String::with_capacity(json.len());
    let mut chars = json.chars().peekable();
    let mut in_string = false;
    let mut escape_next = false;

    while let Some(c) = chars.next() {
        if escape_next {
            result.push(c);
            escape_next = false;
            continue;
        }

        if c == '\\' && in_string {
            result.push(c);
            escape_next = true;
            continue;
        }

        if c == '"' {
            in_string = !in_string;
            result.push(c);
            continue;
        }

        if in_string {
            result.push(c);
            continue;
        }

        if c == '/' {
            match chars.peek() {
                Some(&'/') => {
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        chars.next();
                    }
                }
                Some(&'*') => {
                    chars.next();
                    loop {
                        match chars.next() {
                            Some('*') => {
                                if let Some(&'/') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                            None => break,
                            _ => {}
                        }
                    }
                }
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_line_comments() {
        let input = r#"{
            // This is a comment
            "key": "value"
        }"#;
        let output = strip_comments(input);
        assert!(!output.contains("This is a comment"));
        assert!(output.contains("\"key\""));
    }

    #[test]
    fn test_strip_block_comments() {
        let input = r#"{
            /* Block
               comment */
            "key": "value"
        }"#;
        let output = strip_comments(input);
        assert!(!output.contains("Block"));
        assert!(output.contains("\"key\""));
    }

    #[test]
    fn test_preserve_string_content() {
        let input = r#"{
            "url": "http://example.com // not a comment"
        }"#;
        let output = strip_comments(input);
        assert!(output.contains("// not a comment"));
    }
}
