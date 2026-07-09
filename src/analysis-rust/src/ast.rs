use serde_json::Value;

#[derive(Debug)]
pub enum AstNode {
    Object(Vec<(String, AstNode)>),
    Array(Vec<AstNode>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub struct SecurityAnalyzer;

impl SecurityAnalyzer {
    pub fn parse_json(raw_body: &[u8]) -> Option<AstNode> {
        let val: Value = serde_json::from_slice(raw_body).ok()?;
        Some(Self::convert_value(&val))
    }

    fn convert_value(val: &Value) -> AstNode {
        match val {
            Value::Null => AstNode::Null,
            Value::Bool(b) => AstNode::Bool(*b),
            Value::Number(n) => AstNode::Number(n.as_f64().unwrap_or(0.0)),
            Value::String(s) => AstNode::String(s.clone()),
            Value::Array(arr) => {
                AstNode::Array(arr.iter().map(Self::convert_value).collect())
            }
            Value::Object(obj) => {
                let fields = obj.iter()
                    .map(|(k, v)| (k.clone(), Self::convert_value(v)))
                    .collect();
                AstNode::Object(fields)
            }
        }
    }

    pub fn inspect_node(node: &AstNode) -> bool {
        match node {
            AstNode::String(s) => {
                let s_lower = s.to_lowercase();
                if s_lower.contains("select ") || s_lower.contains("union ") || s_lower.contains("' or '") {
                    return false; 
                }
                true
            }
            AstNode::Object(fields) => {
                for (key, val) in fields {
                    if key.to_lowercase() == "password" || key.to_lowercase() == "pwd" {
                        if let AstNode::String(s) = val {
                            if s.len() < 4 { return false; }
                        }
                    }
                    if !Self::inspect_node(val) { return false; }
                }
                true
            }
            AstNode::Array(items) => {
                for item in items {
                    if !Self::inspect_node(item) { return false; }
                }
                true
            }
            _ => true,
        }
    }

    pub fn inspect_url(url: &str) -> bool {
        let url_lower = url.to_lowercase();

        let traversal_patterns = ["../", "..\\", "..%2f", "%2e%2e%2f", "/etc/passwd", "win.ini", "boot.ini"];
        for pattern in traversal_patterns {
            if url_lower.contains(pattern) {
                return false;
            }
        }

        let xss_patterns = ["<script", "javascript:", "onerror=", "onload=", "<img", "alert("];
        for pattern in xss_patterns {
            if url_lower.contains(pattern) {
                return false;
            }
        }

        true
    }
}