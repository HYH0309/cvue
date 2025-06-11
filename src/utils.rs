use crate::error::ActionError;
use regex::Regex;
use url::Url;

/// 标准化 GitHub URL (owner/repo → https URL)
pub fn normalize_gh_url(repo: &str) -> Result<String, ActionError> {
    if let Some(caps) = Regex::new(r"^([\w.-]+)/([\w.-]+)$")?.captures(repo) {
        Ok(format!("https://github.com/{}/{}.git", &caps[1], &caps[2]))
    } else if let Ok(url) = Url::parse(repo) {
        if url.host_str() == Some("github.com") {
            Ok(repo.to_string())
        } else {
            Err(ActionError::Other("无效的 GitHub URL".into()))
        }
    } else {
        Err(ActionError::Other("无效的 GitHub URL".into()))
    }
}

/// 添加 GitHub 认证信息
pub fn add_gh_auth(url: &str, token: Option<&str>) -> Result<String, ActionError> {
    match token {
        Some(t) if !url.contains('@') && url.starts_with("https://") => {
            let mut authed = format!("https://{}@{}", t, &url[8..]);
            if !authed.ends_with(".git") {
                authed.push_str(".git");
            }
            Ok(authed)
        },
        Some(_) => Err(ActionError::Other("URL 已包含认证信息".into())),
        None => Ok(url.to_string()),
    }
}

/// 验证项目名称
pub fn check_name(name: &str) -> Result<(), ActionError> {
    const RESERVED: &[&str] = &["con", "prn", "aux", "nul",
        "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8", "com9",
        "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9"];
    match name {
        "" => Err(ActionError::Other("项目名称不能为空".into())),
        n if n.len() > 255 => Err(ActionError::Other("名称过长(最多255字符)".into())),
        n if n.chars().any(|c| matches!(c, '<'|'>'|':'|'"'|'|'|'?'|'*'|'\\'|'/')) =>
            Err(ActionError::Other("包含无效字符".into())),
        n if RESERVED.contains(&n.to_lowercase().as_str()) =>
            Err(ActionError::Other("保留名称不能使用".into())),
        _ => Ok(())
    }
}