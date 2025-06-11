use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;

/// 模板文件路径，统一管理
pub const TEMPLATE_PATH: &str = "templates.yaml";

/// 模板结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub alias: String,
    pub repo: String,
    pub description: String,
    pub is_default: bool,
}

/// 加载模板
pub fn load_templates() -> Vec<TemplateInfo> {
    fs::read_to_string(TEMPLATE_PATH)
        .ok()
        .and_then(|content| serde_yaml::from_str(&content).ok())
        .unwrap_or_default()
}

/// 保存模板
pub fn save_templates(templates: &[TemplateInfo]) {
    if let Ok(content) = serde_yaml::to_string(templates) {
        let _ = fs::write(TEMPLATE_PATH, content);
    }
}

/// 根据别名查找模板
pub fn find_by_alias<'a>(templates: &'a [TemplateInfo], alias: &str) -> Option<&'a TemplateInfo> {
    templates.iter().find(|t| t.alias == alias)
}

/// 查找默认模板
pub fn get_default_template(templates: &[TemplateInfo]) -> Option<&TemplateInfo> {
    templates.iter().find(|t| t.is_default)
}

/// 交互式展示模板
pub fn show_templates_interactive() {
    let templates = load_templates();
    crate::ui::print_title("交互式模板管理");

    if templates.is_empty() {
        crate::ui::print_warning("没有模板，请先添加或运行 'cvue init' 初始化常用模板。");
        return;
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
        .collect(); // 选择模板
    println!("请选择要操作的模板：");
    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .items(&template_options)
        .default(0)
        .interact();

    if let Ok(index) = selection {
        let template = &templates[index];

        // 显示模板详情
        crate::ui::print_template_detail(template); // 提供操作选项
        let options = vec!["克隆此模板", "删除此模板", "更新此模板", "返回"];
        println!("\n请选择要执行的操作：");
        let operation = Select::with_theme(&theme)
            .items(&options)
            .default(0)
            .interact();

        if let Ok(op) = operation {
            match op {
                0 => {
                    // 克隆模板
                    println!("请输入目标目录名称：");
                    let target: String = Input::with_theme(&theme)
                        .interact_text()
                        .unwrap_or_else(|_| "project".to_string());

                    crate::commands::clone::run(crate::commands::clone::CloneArgs {
                        template: Some(template.alias.clone()),
                        target: Some(target),
                        token: None,
                    })
                    .unwrap_or_else(|e| {
                        crate::ui::print_error(&format!("克隆失败: {}", e));
                    });
                }
                1 => {
                    // 删除模板
                    let confirm =
                        crate::ui::confirm(&format!("确定要删除模板 '{}'吗?", template.alias))
                            .unwrap_or(false);
                    if confirm {
                        remove_template(template.alias.clone());
                    } else {
                        crate::ui::print_info("已取消删除操作");
                    }
                }
                2 => {
                    // 更新模板
                    println!("请选择要更新的属性：");
                    let update_options = vec!["仓库地址", "描述", "默认状态", "全部更新", "取消"];
                    let update_choice = Select::with_theme(&theme)
                        .items(&update_options)
                        .default(0)
                        .interact();

                    if let Ok(update_op) = update_choice {
                        match update_op {
                            0 => {
                                // 更新仓库
                                let repo: String = Input::with_theme(&theme)
                                    .with_prompt("请输入新的仓库地址")
                                    .with_initial_text(&template.repo)
                                    .interact_text()
                                    .unwrap_or_else(|_| template.repo.clone());
                                update_template(template.alias.clone(), Some(repo), None, None);
                            }
                            1 => {
                                // 更新描述
                                let desc: String = Input::with_theme(&theme)
                                    .with_prompt("请输入新的描述")
                                    .with_initial_text(&template.description)
                                    .interact_text()
                                    .unwrap_or_else(|_| template.description.clone());
                                update_template(template.alias.clone(), None, Some(desc), None);
                            }
                            2 => {
                                // 更新默认状态
                                let is_default = Confirm::with_theme(&theme)
                                    .with_prompt("设为默认模板?")
                                    .default(template.is_default)
                                    .interact()
                                    .unwrap_or(template.is_default);
                                update_template(
                                    template.alias.clone(),
                                    None,
                                    None,
                                    Some(is_default),
                                );
                            }
                            3 => {
                                // 全部更新
                                let repo: String = Input::with_theme(&theme)
                                    .with_prompt("请输入新的仓库地址")
                                    .with_initial_text(&template.repo)
                                    .interact_text()
                                    .unwrap_or_else(|_| template.repo.clone());
                                let desc: String = Input::with_theme(&theme)
                                    .with_prompt("请输入新的描述")
                                    .with_initial_text(&template.description)
                                    .interact_text()
                                    .unwrap_or_else(|_| template.description.clone());

                                let is_default = Confirm::with_theme(&theme)
                                    .with_prompt("设为默认模板?")
                                    .default(template.is_default)
                                    .interact()
                                    .unwrap_or(template.is_default);

                                update_template(
                                    template.alias.clone(),
                                    Some(repo),
                                    Some(desc),
                                    Some(is_default),
                                );
                            }
                            _ => {
                                crate::ui::print_info("已取消更新操作");
                            }
                        }
                    }
                }
                _ => {
                    // 返回
                    crate::ui::print_info("已退出");
                }
            }
        }
    }
}

/// 展示模板
pub fn show_templates() {
    let templates = load_templates();
    crate::ui::print_title("可用模板");
    crate::ui::print_template_table(&templates);
}

/// 添加模板
pub fn add_template(alias: String, repo: String, description: String, is_default: bool) {
    let mut templates = load_templates();

    let spinner = crate::ui::create_spinner("正在检查模板...");

    if find_by_alias(&templates, &alias).is_some() {
        spinner.finish_and_clear();
        crate::ui::print_warning(&format!("模板 '{}' 已存在，不能重复添加", alias));
        return;
    }

    // 若设为默认，把其它全部设为非默认
    if is_default {
        templates.iter_mut().for_each(|t| t.is_default = false);
    }

    templates.push(TemplateInfo {
        alias: alias.clone(),
        repo,
        description,
        is_default,
    });

    spinner.set_message("正在保存模板...");
    save_templates(&templates);
    spinner.finish_and_clear();

    crate::ui::print_success(&format!("模板 '{}' 添加成功！", alias));
}

/// 删除模板
pub fn remove_template(alias: String) {
    let mut templates = load_templates();
    let old_len = templates.len();

    let spinner = crate::ui::create_spinner(&format!("正在删除模板 '{}'...", alias));

    templates.retain(|t| t.alias != alias);

    if templates.len() == old_len {
        spinner.finish_and_clear();
        crate::ui::print_warning(&format!("未找到别名为 '{}' 的模板", alias));
    } else {
        save_templates(&templates);
        spinner.finish_and_clear();
        crate::ui::print_success(&format!("模板 '{}' 已删除", alias));
    }
}

/// 更新模板
pub fn update_template(
    alias: String,
    repo: Option<String>,
    description: Option<String>,
    is_default: Option<bool>,
) {
    let mut templates = load_templates();
    let mut found = false;

    let spinner = crate::ui::create_spinner(&format!("正在更新模板 '{}'...", alias));

    // 若设为默认，全部取消默认
    if matches!(is_default, Some(true)) {
        templates.iter_mut().for_each(|t| t.is_default = false);
    }

    for tpl in &mut templates {
        if tpl.alias == alias {
            if let Some(r) = repo {
                tpl.repo = r;
            }
            if let Some(d) = description {
                tpl.description = d;
            }
            if let Some(df) = is_default {
                tpl.is_default = df;
            }
            found = true;
            break;
        }
    }

    if found {
        save_templates(&templates);
        spinner.finish_and_clear();
        crate::ui::print_success(&format!("模板 '{}' 更新成功", alias));
    } else {
        spinner.finish_and_clear();
        crate::ui::print_warning(&format!("未找到别名为 '{}' 的模板", alias));
    }
}

/// 获取模板
pub fn get_template(alias: String) {
    let templates = load_templates();

    let spinner = crate::ui::create_spinner(&format!("正在查找模板 '{}'...", alias));

    match find_by_alias(&templates, &alias) {
        Some(tpl) => {
            spinner.finish_and_clear();
            crate::ui::print_template_detail(tpl);
        }
        None => {
            spinner.finish_and_clear();
            crate::ui::print_warning(&format!("未找到别名为 '{}' 的模板", alias));
        }
    }
}
