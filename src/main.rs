mod cli;
mod commands;
mod error;
mod utils;
mod ui;

fn main() {
    // 显示应用标志
    ui::print_logo();
    
    // 运行CLI
    if let Err(e) = cli::run() {
        ui::print_error(&format!("错误: {}", e));
        std::process::exit(1);
    }
}