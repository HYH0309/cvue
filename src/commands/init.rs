use crate::commands::list::{find_by_alias, load_templates, save_templates, TemplateInfo};
use crate::error::ActionError;
use colored::*;

/// 初始化命令参数结构体
pub struct InitArgs {
    /// 是否强制覆盖已有模板
    pub force: bool,
}

/// 常用Vue模板列表
const DEFAULT_TEMPLATES: &[(&str, &str, &str, bool)] = &[
    (
        "vue3-vite",
        "vuejs/create-vue",
        "Vue 3 + Vite官方模板",
        true,
    ),
    ("vue2", "vuejs/vue-cli", "Vue 2官方CLI模板", false),
    ("nuxt3", "nuxt/starter", "Nuxt 3入门模板", false),
    (
        "vue3-ts",
        "vuejs-templates/webpack-simple",
        "Vue 3 + TypeScript模板",
        false,
    ),
    (
        "vue-element",
        "PanJiaChen/vue-element-admin",
        "基于Element UI的后台管理模板",
        false,
    ),
];

/// 运行初始化命令
pub fn run(args: InitArgs) -> Result<(), ActionError> {
    let mut templates = load_templates();
    let mut added_count = 0;
    let mut updated_count = 0;

    crate::ui::print_title("初始化常用Vue模板");

    let spinner = crate::ui::create_spinner("准备模板数据中...");
    std::thread::sleep(std::time::Duration::from_millis(800)); // 增加视觉效果
    spinner.finish_and_clear();

    let pb = indicatif::ProgressBar::new(DEFAULT_TEMPLATES.len() as u64);
    pb.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
        .unwrap()
        .progress_chars("#>-"));

    for (alias, repo, description, is_default) in DEFAULT_TEMPLATES {
        pb.set_message(format!("处理模板: {}", alias));

        // 检查是否已存在
        if let Some(_) = find_by_alias(&templates, alias) {
            if !args.force {
                crate::ui::print_warning(&format!("已跳过：模板 '{}' 已存在", alias));
                pb.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(300)); // 慢一点，便于用户查看
                continue;
            }

            // 更新现有模板
            // 如果当前模板要设为默认，先取消其他模板的默认状态
            if *is_default {
                templates.iter_mut().for_each(|t| t.is_default = false);
            }

            for tpl in &mut templates {
                if &tpl.alias == alias {
                    tpl.repo = repo.to_string();
                    tpl.description = description.to_string();
                    tpl.is_default = *is_default;
                    break;
                }
            }
            updated_count += 1;
            crate::ui::print_info(&format!("已更新：模板 '{}'", alias));
        } else {
            // 如果当前模板要设为默认，先取消其他模板的默认状态
            if *is_default {
                templates.iter_mut().for_each(|t| t.is_default = false);
            }

            // 添加新模板
            templates.push(TemplateInfo {
                alias: alias.to_string(),
                repo: repo.to_string(),
                description: description.to_string(),
                is_default: *is_default,
            });
            added_count += 1;
            crate::ui::print_success(&format!("已添加：模板 '{}'", alias));
        }

        pb.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(200)); // 添加延迟以增强视觉效果
    }

    pb.finish_and_clear();
    // 保存更改
    let save_spinner = crate::ui::create_spinner("保存模板配置...");
    save_templates(&templates);
    std::thread::sleep(std::time::Duration::from_millis(800)); // 增强视觉效果
    save_spinner.finish_with_message("模板配置已保存".to_string());

    crate::ui::print_title("初始化完成");
    println!("{} 添加了 {} 个模板", "✓".bright_green(), added_count);
    println!("{} 更新了 {} 个模板", "✓".bright_green(), updated_count);

    // 立即显示所有模板
    if added_count > 0 || updated_count > 0 {
        println!();
        crate::ui::print_info("已添加的模板如下:");
        crate::ui::print_template_table(&templates);
    }

    Ok(())
}
