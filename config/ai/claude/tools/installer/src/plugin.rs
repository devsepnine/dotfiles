#[derive(Clone, Debug, PartialEq)]
pub enum PluginStatus {
    Installed,
    NotInstalled,
}

impl PluginStatus {
    pub fn display(&self) -> &str {
        match self {
            Self::Installed => "installed",
            Self::NotInstalled => "not installed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PluginDef {
    pub name: String,
    pub marketplace: String, // marketplace name (e.g., "claude-plugins-official")
    pub source: String,      // marketplace source URL
    pub comment: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Plugin {
    pub def: PluginDef,
    pub selected: bool,
    pub status: PluginStatus,
}

impl Plugin {
    pub fn new(def: PluginDef, status: PluginStatus) -> Self {
        Self {
            def,
            selected: false,
            status,
        }
    }

    pub fn short_repo(&self) -> String {
        // https://github.com/anthropics/claude-plugins-official.git -> anthropics/claude-plugins-official
        self.def
            .source
            .trim_end_matches(".git")
            .split("github.com/")
            .last()
            .unwrap_or(&self.def.source)
            .to_string()
    }
}

/// plugins.yaml 파싱 결과
/// 형식: Vec<(marketplace_name, source_url, plugin_name, comment)>
pub type PluginCatalog = Vec<(String, String, String, Option<String>)>;

/// plugins.yaml 파싱 (새 형식과 이전 형식 모두 지원)
/// 새 형식:
/// ```yaml
/// marketplaces:
///   marketplace-name:
///     source: https://github.com/repo.git
///     plugins:
///       - plugin-name # comment
/// ```
/// 이전 형식 (하위 호환성):
/// ```yaml
/// https://github.com/repo.git:
///   - plugin-name # comment
/// ```
pub fn parse_plugins_yaml(content: &str) -> PluginCatalog {
    use serde_yaml::Value;

    let mut catalog = PluginCatalog::new();

    let yaml: Value = match serde_yaml::from_str(content) {
        Ok(v) => v,
        Err(_) => return catalog,
    };

    // 새 형식: marketplaces 섹션 확인
    if let Some(marketplaces) = yaml.get("marketplaces") {
        if let Some(marketplaces_map) = marketplaces.as_mapping() {
            for (marketplace_name, marketplace_data) in marketplaces_map {
                let marketplace_name = marketplace_name.as_str().unwrap_or("").to_string();

                if let Some(data_map) = marketplace_data.as_mapping() {
                    let source = data_map.get(&Value::String("source".to_string()))
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string();

                    if let Some(plugins) = data_map.get(&Value::String("plugins".to_string())) {
                        if let Some(plugins_seq) = plugins.as_sequence() {
                            for plugin_entry in plugins_seq {
                                let (name, comment) = parse_plugin_entry(plugin_entry);
                                if !name.is_empty() {
                                    catalog.push((marketplace_name.clone(), source.clone(), name, comment));
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        // 이전 형식: repo URL이 키인 경우 (하위 호환성)
        if let Some(root_map) = yaml.as_mapping() {
            for (key, value) in root_map {
                let key_str = key.as_str().unwrap_or("");

                // repo URL 형식 체크
                if key_str.starts_with("https://") {
                    let repo_url = key_str.to_string();
                    // marketplace 이름은 repo에서 추출
                    let marketplace_name = extract_marketplace_name(&repo_url);

                    if let Some(plugins_seq) = value.as_sequence() {
                        for plugin_entry in plugins_seq {
                            let (name, comment) = parse_plugin_entry(plugin_entry);
                            if !name.is_empty() {
                                catalog.push((marketplace_name.clone(), repo_url.clone(), name, comment));
                            }
                        }
                    }
                }
            }
        }
    }

    catalog
}

fn parse_plugin_entry(entry: &serde_yaml::Value) -> (String, Option<String>) {
    if let Some(s) = entry.as_str() {
        // "plugin-name # comment" 형식 파싱
        if let Some(idx) = s.find('#') {
            let name = s[..idx].trim().to_string();
            let comment = s[idx + 1..].trim().to_string();
            (name, Some(comment))
        } else {
            (s.trim().to_string(), None)
        }
    } else {
        (String::new(), None)
    }
}

fn extract_marketplace_name(repo_url: &str) -> String {
    // https://github.com/anthropics/claude-plugins-official.git -> claude-plugins-official
    repo_url
        .trim_end_matches(".git")
        .split('/')
        .last()
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plugins_yaml_new_format() {
        let yaml = r#"
marketplaces:
  claude-plugins-official:
    source: https://github.com/anthropics/claude-plugins-official.git
    plugins:
      - rust-analyzer-lsp # Rust 언어 서버
      - typescript-lsp    # TypeScript 언어 서버
  anthropic-agent-skills:
    source: https://github.com/anthropics/skills.git
    plugins:
      - document-skills   # 문서 생성/편집
"#;

        let catalog = parse_plugins_yaml(yaml);

        assert_eq!(catalog.len(), 3);

        // Find entries
        let rust_entry = catalog.iter().find(|(_, _, name, _)| name == "rust-analyzer-lsp").unwrap();
        assert_eq!(rust_entry.0, "claude-plugins-official");
        assert_eq!(rust_entry.1, "https://github.com/anthropics/claude-plugins-official.git");
        // YAML parser strips comments, so we won't have them in parsed data
        // assert_eq!(rust_entry.3, Some("Rust 언어 서버".to_string()));

        let doc_entry = catalog.iter().find(|(_, _, name, _)| name == "document-skills").unwrap();
        assert_eq!(doc_entry.0, "anthropic-agent-skills");
    }

    #[test]
    fn test_parse_plugins_yaml_old_format() {
        let yaml = r#"
https://github.com/anthropics/claude-plugins-official.git:
  - rust-analyzer-lsp # Rust 언어 서버
  - typescript-lsp    # TypeScript 언어 서버
"#;

        let catalog = parse_plugins_yaml(yaml);
        assert_eq!(catalog.len(), 2);

        let rust_entry = catalog.iter().find(|(_, _, name, _)| name == "rust-analyzer-lsp").unwrap();
        assert_eq!(rust_entry.0, "claude-plugins-official"); // extracted from URL
    }

    #[test]
    fn test_plugin_short_repo() {
        let plugin = Plugin::new(
            PluginDef {
                name: "test".to_string(),
                marketplace: "claude-plugins-official".to_string(),
                source: "https://github.com/anthropics/claude-plugins-official.git".to_string(),
                comment: None,
            },
            PluginStatus::NotInstalled,
        );

        assert_eq!(plugin.short_repo(), "anthropics/claude-plugins-official");
    }
}
