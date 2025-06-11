# cvue：一个前端用Rust自虐的血泪史 💻💉

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Stars](https://img.shields.io/github/stars/HYH0309/cvue?style=social)](https://github.com/HYH0309/cvue)
[![Version](https://img.shields.io/badge/version-0.1.0-blue)](https://crates.io/crates/cvue)

*用Rust写的Vue模板管理工具，前端程序员的自我折磨之路*  
*(开源许可证：反正您也不会真的用这破玩意)*

</div>

## 📋 目录

- [cvue：一个前端用Rust自虐的血泪史 💻💉](#cvue一个前端用rust自虐的血泪史-)
  - [📋 目录](#-目录)
  - [作者自白](#作者自白)
  - [功能列表（自黑版）](#功能列表自黑版)
  - [安装说明](#安装说明)
  - [使用指南](#使用指南)
    - [基础命令](#基础命令)
    - [命令参数](#命令参数)
    - [示例工作流](#示例工作流)
  - [开发血泪史](#开发血泪史)
    - [1. 与Rust编译器的日常搏斗](#1-与rust编译器的日常搏斗)
    - [2. 前端做CLI UI的耻辱](#2-前端做cli-ui的耻辱)
    - [3. 类型偏执晚期症状](#3-类型偏执晚期症状)
  - [使用建议](#使用建议)
  - [卸载指南](#卸载指南)
  - [贡献指南](#贡献指南)
  - [系统要求](#系统要求)
  - [版权和致谢](#版权和致谢)
  - [常见问题（FAQ）](#常见问题faq)
  - [进阶玩法](#进阶玩法)
  - [真实用户反馈](#真实用户反馈)
  - [相关项目推荐](#相关项目推荐)
  - [交流与支持](#交流与支持)

## 作者自白

```rust
struct Developer {
    name: String, // "一个不务正业的前端"
    condition: String, // "Rust编译错误PTSD"
    projects: Vec<String>, // ["无用的CLI工具"]
}
```

## 功能列表（自黑版）

- ✨ **模板下载** - 用Rust重写了`git clone`（但慢3倍）
- 🚀 **彩色终端** - 引入37个crate就为让`console.log`变彩虹色
- 📦 **依赖管理** - 把`package.json`的依赖问题升级成`Cargo.lock`的噩梦
- 🔄 **跨平台支持** - 虽然您这辈子只会用Win开发
- 🎭 **简短别名** - 为全部命令提供简短别名，让你少敲几个字（这也算功能？）

## 安装说明

```bash
# 自虐套餐
cargo install cvue
```

*预计耗时：*

- 下载：2分钟
- 编译：足够您后悔3次人生选择
- 调试：直到放弃

## 使用指南

> 您真的打算用这个工具？好吧，那就继续...

### 基础命令

```bash
# 查看帮助
cvue help  # 或者用简写 cvue -h

# 列出所有模板
cvue show   # 简写：cvue s 或 cvue list

# 交互式查看模板
cvue show -i

# 添加新模板
cvue add -a 模板别名 -r 仓库地址 -e "这是个神奇的模板" -d  # -d 表示设为默认
# 简写：cvue a -a 模板别名 ...

# 删除模板
cvue remove -a 模板别名  # 简写：cvue rm 或 cvue r 

# 获取模板详情
cvue get -a 模板别名  # 简写：cvue g

# 克隆模板（会提示选择）
cvue clone  # 简写：cvue c

# 克隆指定模板到指定目录
cvue clone 模板别名 -t ./my-project

# 初始化默认模板集
cvue init  # 简写：cvue i
```

### 命令参数

| 命令 | 参数 | 描述 | 是否必需 |
|------|------|------|----------|
| `show` | `-i, --interactive` | 启用交互模式 | 否 |
| `add` | `-a, --alias <名称>` | 模板别名 | 是 |
|  | `-r, --repo <地址>` | 仓库URL | 是 |
|  | `-e, --description <描述>` | 模板描述 | 是 |
|  | `-d, --default` | 设为默认模板 | 否 |
| `remove` | `-a, --alias <名称>` | 要删除的模板别名 | 是 |
| `get` | `-a, --alias <名称>` | 要查看的模板别名 | 是 |
| `clone` | `[模板别名]` | 要克隆的模板 | 否 |
|  | `-t, --target <目录>` | 目标目录 | 否 |
|  | `-k, --token <TOKEN>` | Git令牌 | 否 |
| `init` | `-f, --force` | 强制覆盖已有模板 | 否 |

### 示例工作流

```bash
# 第一次使用，初始化默认模板
cvue i

# 查看可用模板
cvue s

# 使用Vue 3模板创建新项目
cvue c vue3-vite -t ./my-new-app

# 添加自己的模板
cvue a -a my-template -r https://github.com/me/my-template -e "我的自定义模板"

# 设为默认模板
cvue u my-template -d true
```

## 开发血泪史

### 1. 与Rust编译器的日常搏斗

```rust
// src/commands/list.rs 真实片段
match templates {
    Ok(_) => unreachable!(), // 理想情况
    Err(e) => { // 现实情况
        eprintln!("错误 {}：", e); // 每天看100遍
        process::exit(1); // 最稳定的功能
    }
}
```

开发时间分配：

```mermaid
pie
    title 时间都去哪了
    "写代码" : 10
    "查文档" : 30
    "解决编译错误" : 60
```

### 2. 前端做CLI UI的耻辱

```rust
// src/ui.rs 的荒谬现实
fn show_progress() {
    // 花了3天就为这个进度条
    // 用户根本不care的动画效果
    // 比实际功能代码多10倍的UI代码
}
```

成果对比：

| 功能 | 理性实现 | 我的实现 | 复杂度倍数 |
|------|----------|----------|------------|
| 进度提示 | `println!` | 动画进度条 | 100x |
| 错误显示 | `eprintln!` | 彩色格式化 | 50x |

### 3. 类型偏执晚期症状

```rust
// 用火箭筒打蚊子的典范
pub fn download_template(url: &str) 
    -> Result<Option<Arc<Mutex<Template>>, Box<dyn Error>> {
    // 就下个模板而已...
}
```

## 使用建议

```bash
# 推荐替代方案
git clone 正经模板仓库

# 如果非要自虐
cvue init -f # -f 表示"放弃治疗"
```

## 卸载指南

```bash
cargo uninstall cvue # 卸载这个耻辱
rm -rf ~/.cvue     # 删除所有痕迹
```

## 贡献指南

> 您确定要对这个项目做贡献？真的确定？

1. Fork此仓库（如果您真的觉得值得）
2. 创建您的功能分支 (`git checkout -b feature/我想加的功能`)
3. 提交您的更改 (`git commit -m '加入了某某功能，希望没有搞砸'`)
4. 推送到分支 (`git push origin feature/我想加的功能`)
5. 创建Pull Request（然后等着被吐槽代码风格）

## 系统要求

- Rust 1.75+ (或者任何能编译这堆依赖的版本)
- 足够耐心等待编译
- 足够内存容纳Cargo.lock
- 建议：心理医生联系方式（在编译失败时使用）

## 版权和致谢

本项目采用[MIT许可证](LICENSE)，您可以随意使用，但请不要告诉别人是从哪抄的。

特别感谢：

- Rust编译器：教会我怀疑人生
- VS Code：在报错信息中提供了心灵慰藉
- 堆栈溢出：没有你，这个项目根本不可能存在

---

## 常见问题（FAQ）

<details>
<summary>Q: 为什么我编译这么慢？</summary>

A: 你以为是你电脑的问题，其实是Rust的哲学：**“慢工出细活”**。建议编译时去泡杯咖啡，或者思考一下人生。

</details>

<details>
<summary>Q: 为什么我运行`cvue`报错？</summary>

A: 你以为是你操作的问题，其实是作者没测全。建议多试几次，或者直接提个issue（作者可能会装死）。

</details>

<details>
<summary>Q: 支持Mac/Linux吗？</summary>

A: 理论上支持，实际上作者只在Windows上自虐过。如果你能跑起来，欢迎来炫耀。

</details>

---

## 进阶玩法

- **自定义模板**  
  你可以随意添加任何Git仓库作为模板，甚至可以加上你老板的头像（只要你开心）。
- **一键切换默认模板**  
  用`cvue u <别名> -d true`，让你最爱的模板永远排在第一。
- **命令行彩蛋**  
  有些命令会随机输出彩色表情，别问，问就是写着玩。

---

## 真实用户反馈

> “用了cvue之后，我的Rust水平倒退了半年。”  
> —— 某前端同事
>
> “唯一能让我主动写文档的CLI工具。”  
> —— HYH 本人
>
> “建议直接用git clone。”  
> —— 路人甲

---

## 相关项目推荐

- [vue-cli](https://github.com/vuejs/vue-cli)  
  官方正经工具，适合不想自虐的你。
- [create-vue](https://github.com/vuejs/create-vue)  
  Vue 3官方脚手架，速度快，体验好。
- [cargo-generate](https://github.com/cargo-generate/cargo-generate)  
  Rust模板生成工具，适合喜欢折腾的Rustacean。

---

## 交流与支持

- 有问题请提[Issue](https://github.com/HYH0309/cvue/issues)
- 欢迎PR，哪怕只是改个错别字
- 觉得有趣请点个Star，作者会更有动力继续自虐

---

<div align="center">
  <img src="https://github.githubassets.com/images/icons/emoji/unicode/1f389.png" width="32" alt="party" />
  <br/>
  <b>感谢每一位看到这里的勇士！</b>
</div>
