use crate::commands::list::{load_templates, get_default_template, find_by_alias};
use crate::utils::*;
use crate::error::ActionError;
use colored::*;
use std::path::PathBuf;
use dialoguer::{theme::ColorfulTheme, Select, Input};

/// 克隆命令参数结构体
pub struct CloneArgs {
    pub template: Option<String>, // 别名或repo
    pub target: Option<String>,
    pub token: Option<String>,
}

/// 运行克隆命令
pub fn run(args: CloneArgs) -> Result<(), ActionError> {
    crate::ui::print_title("克隆Vue模板");
    
    // 如果未提供模板，进入交互式模式
    if args.template.is_none() {
        return run_interactive(&args);
    }
    
    let spinner = crate::ui::create_spinner("加载模板配置...");
    let templates = load_templates();
    std::thread::sleep(std::time::Duration::from_millis(500));
    spinner.finish_and_clear();

    // 选择模板仓库
    let repo = match &args.template {
        Some(name) => {
            let template_spinner = crate::ui::create_spinner(&format!("查找模板 '{}'...", name));
            std::thread::sleep(std::time::Duration::from_millis(500));
            
            let result = find_by_alias(&templates, name)
                .map(|tpl| {
                    template_spinner.finish_and_clear();
                    crate::ui::print_success(&format!("使用模板: {} ({})", tpl.alias, tpl.description));
                    tpl.repo.as_str()
                })
                .unwrap_or_else(|| {
                    template_spinner.finish_and_clear();
                    crate::ui::print_info(&format!("未找到模板 '{}', 将直接使用作为仓库地址", name));
                    name
                })
                .to_string();
            
            result
        }
        None => {
            let default_spinner = crate::ui::create_spinner("查找默认模板...");
            std::thread::sleep(std::time::Duration::from_millis(500));
            
            match get_default_template(&templates) {
                Some(tpl) => {
                    default_spinner.finish_and_clear();
                    crate::ui::print_success(&format!("使用默认模板: {} ({})", tpl.alias, tpl.description));
                    tpl.repo.clone()
                }
                None => {
                    default_spinner.finish_and_clear();
                    return Err(ActionError::Other("没有默认模板，请先使用 'cvue init' 初始化模板或指定要使用的模板".into()));
                }
            }
        }
    };

    // 设置目标目录名
    let dir_name = args
        .target
        .as_deref()
        .unwrap_or_else(|| repo.split('/').last().unwrap_or("template"));
    
    let check_spinner = crate::ui::create_spinner(&format!("检查项目名称 '{}'...", dir_name));
    check_name(dir_name)?;
    let target_path = PathBuf::from(dir_name);
    check_spinner.finish_and_clear();
    
    // 目标目录存在时交互询问
    if target_path.exists() {
        let confirm = crate::ui::confirm(&format!("目录 '{}' 已存在。是否覆盖?", target_path.display()))?;
        if !confirm {
            crate::ui::print_info("已取消操作");
            return Ok(());
        }
        
        let remove_spinner = crate::ui::create_spinner(&format!("正在删除目录 '{}'...", target_path.display()));
        std::fs::remove_dir_all(&target_path)?;
        remove_spinner.finish_and_clear();
    }

    // 处理仓库地址
    let url_spinner = crate::ui::create_spinner("准备仓库地址...");
    let url = normalize_gh_url(&repo)?;
    let url = add_gh_auth(&url, args.token.as_deref())?;
    url_spinner.finish_and_clear();
    
    crate::ui::print_info(&format!("将从 {} 克隆到 {}", 
        url.replace(args.token.as_deref().unwrap_or(""), "****"), 
        target_path.display()));

    // 执行 git clone
    let pb = crate::ui::create_spinner(&format!("正在克隆到 {}...", target_path.display()));
    
    let status = std::process::Command::new("git")
        .arg("clone")
        .arg(&url)
        .arg(&target_path)
        .status()
        .map_err(|e| ActionError::Other(format!("无法执行 git: {}", e)))?;
    
    if status.success() {
        pb.finish_and_clear();
        crate::ui::print_title("克隆成功");
        println!("{} 项目已克隆到: {}", "✓".bright_green(), target_path.display().to_string().bright_green());
        println!("\n{} {}", "▶".bright_green(), "开始使用:".bright_green());
        println!("  cd {}", dir_name);
        println!("  npm install");
        println!("  npm run dev");
        Ok(())
    } else {
        pb.finish_and_clear();
        Err(ActionError::Other(format!("git clone 失败，返回码: {:?}", status.code())))
    }
}

/// 运行交互式模板选择
fn run_interactive(args: &CloneArgs) -> Result<(), ActionError> {
    crate::ui::print_title("交互式模板选择");

    let templates = load_templates();
    if templates.is_empty() {
        crate::ui::print_warning("没有可用的模板，请先运行 'cvue init' 初始化模板库。");
        return Ok(());
    }

    // 准备模板列表
    let template_options: Vec<String> = templates
        .iter()
        .map(|t| {
            format!(
                "{} - {} {}",
                t.alias.bright_cyan(),
                t.description,
                if t.is_default {
                    "[默认]".bright_green()
                } else {
                    "".into()
                }
            )
        })
        .collect();

    // 添加一个选项用于输入自定义仓库
    let mut display_options = template_options.clone();
    display_options.push("输入自定义仓库地址...".bright_yellow().to_string());
    
    // 选择模板
    println!("请选择要克隆的模板：");
    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .items(&display_options)
        .default(0)
        .interact()
        .map_err(|e| ActionError::Other(format!("交互错误: {}", e)))?;

    let template = if selection == templates.len() {
        // 用户选择了自定义仓库
        let repo_url: String = Input::with_theme(&theme)
            .with_prompt("请输入GitHub仓库地址 (user/repo 或 URL)")
            .interact_text()
            .map_err(|e| ActionError::Other(format!("交互错误: {}", e)))?;
        
        Some(repo_url)
    } else {
        // 用户选择了预设模板
        Some(templates[selection].alias.clone())
    };

    // 输入目标目录
    let target: String = Input::with_theme(&theme)
        .with_prompt("请输入目标目录名称")
        .interact_text()
        .map_err(|e| ActionError::Other(format!("交互错误: {}", e)))?;

    // 准备克隆参数
    let clone_args = CloneArgs {
        template,
        target: Some(target),
        token: args.token.clone(),
    };

    // 执行克隆
    run(clone_args)
}