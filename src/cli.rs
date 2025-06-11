use crate::commands::{clone, init, list};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cvue")]
#[command(about = "模板项目管理工具", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 展示所有模板(支持交互式操作) [别名: s, list]
    #[command(alias = "s", alias = "list")]
    Show(ShowArgs),
    /// 添加模板 [别名: a]
    #[command(alias = "a")]
    Add(AddArgs),
    /// 删除模板 [别名: rm, r]
    #[command(alias = "rm", alias = "r")]
    Remove(RemoveArgs),
    /// 更新模板 [别名: u]
    #[command(alias = "u")]
    Update(UpdateArgs),
    /// 获取指定模板详情 [别名: g]
    #[command(alias = "g")]
    Get(GetArgs),
    /// 克隆模板(不提供模板参数时进入交互式选择) [别名: c]
    #[command(alias = "c")]
    Clone(CloneArgs),
    /// 初始化新项目 [别名: i]
    #[command(alias = "i")]
    Init(InitArgs),
}

#[derive(Args)]
pub struct ShowArgs {
    /// 是否使用交互模式
    #[arg(short = 'i', long = "interactive", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub interactive: bool,
}

#[derive(Args)]
pub struct AddArgs {
    /// 别名
    #[arg(short, long)]
    pub alias: String,
    /// 仓库地址
    #[arg(short, long)]
    pub repo: String,
    /// 描述
    #[arg(short = 'e', long)]
    pub description: String,
    /// 是否设为默认
    #[arg(short, long, default_value_t = false)]
    pub default: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// 别名
    #[arg(short, long)]
    pub alias: String,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// 别名
    pub alias: String,
    /// 仓库地址
    #[arg(short = 'r', long)]
    pub repo: Option<String>,
    /// 描述
    #[arg(short = 'e', long)]
    pub description: Option<String>,
    /// 是否设为默认
    #[arg(short, long)]
    pub default: Option<bool>,
}

#[derive(Args)]
pub struct GetArgs {
    /// 别名
    #[arg(short, long)]
    pub alias: String,
}

#[derive(Args)]
pub struct CloneArgs {
    /// 模板别名或仓库地址
    pub template: Option<String>,
    /// 目标目录
    #[arg(short = 't', long)]
    pub target: Option<String>,
    /// Git Token
    #[arg(short = 'k', long)]
    pub token: Option<String>,
}

#[derive(Args)]
pub struct InitArgs {
    /// 强制覆盖已有模板
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Show(args) => {
            if args.interactive {
                list::show_templates_interactive();
            } else {
                list::show_templates();
            }
        }
        Commands::Add(args) => {
            list::add_template(args.alias, args.repo, args.description, args.default);
        }
        Commands::Remove(args) => {
            list::remove_template(args.alias);
        }
        Commands::Update(args) => {
            list::update_template(args.alias, args.repo, args.description, args.default);
        }
        Commands::Get(args) => {
            list::get_template(args.alias);
        }
        Commands::Clone(args) => {
            clone::run(clone::CloneArgs {
                template: args.template,
                target: args.target,
                token: args.token,
            })?;
        }
        Commands::Init(args) => {
            init::run(init::InitArgs { force: args.force })?;
        }
    }
    Ok(())
}
