use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self};
use std::time::Duration;
use tabled::{settings::style::Style, Table, Tabled};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

// Message Components
pub fn print_success(msg: &str) {
    println!("{} {}", "✓".bright_green(), msg.green());
}

pub fn print_error(msg: &str) {
    println!("{} {}", "✗".bright_red(), msg.red());
}

pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".bright_yellow(), msg.yellow());
}

pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".bright_blue(), msg.blue());
}

// Title Component
pub fn print_title(title: &str) {
    let width = title.width() + 4;
    println!("╭{}╮", "─".repeat(width));
    println!(
        "│ {}{} │",
        title.bright_cyan().underline(),
        " ".repeat(title.width())
    );
    println!("╰{}╯", "~".repeat(width));
}

/// 打印详情卡片
///
/// # Arguments
/// * `items` - 键值对列表 (标签, 值)
pub fn print_detail_card(items: &[(&str, String)]) {
    let max_label_width = items
        .iter()
        .map(|(label, _)| label.width())
        .max()
        .unwrap_or(0);

    println!("╭{}╮", "─".repeat(max_label_width + 7));
    for (label, value) in items {
        println!(
            "│ {}{} │ {}",
            label.bright_cyan(),
            " ".repeat(max_label_width - label.width()),
            value
        );
    }
    println!("╰{}╯", "─".repeat(max_label_width + 7));
}

/// 打印分隔线
///
/// # Arguments
/// * `length` - 分隔线长度
/// * `color` - 分隔线颜色
pub fn print_divider(length: usize, color: Color) {
    println!("{}", "─".repeat(length).color(color));
}

#[derive(Tabled)]
struct TemplateDisplay {
    #[tabled(rename = "别名")]
    alias: String,
    #[tabled(rename = "仓库")]
    repo: String,
    #[tabled(rename = "描述")]
    description: String,
    #[tabled(rename = "默认")]
    is_default: String,
}

/// 打印模板信息表格
///
/// # Arguments
/// * `templates` - 模板信息切片
///
/// # 示例
/// ```
/// print_template_table(&templates);
/// ```
pub fn print_template_table(templates: &[crate::commands::list::TemplateInfo]) {
    if templates.is_empty() {
        print_warning("没有模板，请先添加或运行 'cvue init' 初始化常用模板。");
        return;
    }

    // 转换模板数据为显示格式
    let display_templates: Vec<TemplateDisplay> = templates
        .iter()
        .map(|tpl| {
            let desc = if let Some((width, _)) = terminal_size() {
                // 根据终端宽度限制描述长度
                let max_width = (width.0 as usize).min(80) / 3; // 分配1/3宽度给描述
                if tpl.description.chars().count() > max_width {
                    let truncated: String = tpl.description.chars().take(max_width - 3).collect();
                    format!("{}...", truncated)
                } else {
                    tpl.description.clone()
                }
            } else {
                // 默认截断长度
                tpl.description.chars().take(40).collect()
            };

            TemplateDisplay {
                alias: tpl.alias.clone(),
                repo: tpl.repo.clone(),
                description: desc,
                is_default: if tpl.is_default {
                    "✓".bright_green().to_string()
                } else {
                    "".to_string()
                },
            }
        })
        .collect(); // 创建并格式化表格
    let mut table = Table::new(display_templates);
    table.with(Style::modern());

    // 斑马条纹效果
    let output = table.to_string();
    for (i, line) in output.lines().enumerate() {
        if i >= 2 {
            // Skip header and top border
            if i % 2 == 0 {
                println!("{}", line.on_bright_black());
            } else {
                println!("{}", line);
            }
        } else {
            println!("{}", line);
        }
    }
}

/// 打印模板详情
///
/// # Arguments
/// * `template` - 要显示的模板信息
///
/// # 示例
/// ```
/// print_template_detail(&template);
/// ```
pub fn print_template_detail(template: &crate::commands::list::TemplateInfo) {
    print_title(&format!("模板详情: {}", template.alias)); // 使用表格显示详情
    let mut table = Table::new(vec![
        ["别名".bright_cyan().to_string(), template.alias.clone()],
        ["仓库".bright_cyan().to_string(), template.repo.clone()],
        [
            "描述".bright_cyan().to_string(),
            template.description.clone(),
        ],
        [
            "默认".bright_cyan().to_string(),
            if template.is_default {
                "是".bright_green().to_string()
            } else {
                "否".normal().to_string()
            },
        ],
    ]);

    table.with(Style::ascii());
    println!("{}", table);
}

/// 交互式确认
pub fn confirm(message: &str) -> io::Result<bool> {
    let theme = ColorfulTheme::default();
    Confirm::with_theme(&theme)
        .with_prompt(message)
        .default(false)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// 创建进度条
///
/// # Arguments
/// * `message` - 进度条显示的消息
///
/// # 返回
/// 返回配置好的ProgressBar实例
///
/// # 示例
/// ```
/// let spinner = create_spinner("处理中...");
/// // 处理完成后调用 spinner.finish();
/// ```
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// 打印应用标志
///
/// # 示例
/// ```
/// print_logo();
/// ```
pub fn print_logo() {
    println!(
        "{}",
        r#"
  _____                
 / ____|               
| |     __   __   _    _    ___ 
| |     \ \ / /  | |  | |  / _ \
| |____  \ V /   | |__| | |  __/
 \_____|  \_/     \____/   \___|
                                
"#
        .bright_cyan()
    );

    println!("{}", "Vue模板管理CLI工具".bright_green().bold());

    // 版本信息卡片
    print_detail_card(&[
        ("版本", "0.1.0".to_string()),
        ("作者", "HYH".to_string()),
        (
            "仓库",
            "https://github.com/HYH0309/cvue".bright_blue().to_string(),
        ),
        ("许可证", "MIT".to_string()),
    ]);

    print_divider(40, Color::Cyan);
}
