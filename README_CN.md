# MdANSI

[English](./README.md) | **中文**

[![CI](https://img.shields.io/github/actions/workflow/status/justinhuangcode/mdANSI/ci.yml?branch=main&label=CI&style=flat-square)](https://github.com/justinhuangcode/mdANSI/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/mdansi?style=flat-square)](https://crates.io/crates/mdansi)
[![docs.rs](https://img.shields.io/docsrs/mdansi?style=flat-square)](https://docs.rs/mdansi)
[![License](https://img.shields.io/crates/l/mdansi?style=flat-square)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange?style=flat-square)](https://www.rust-lang.org)
[![GitHub Stars](https://img.shields.io/github/stars/justinhuangcode/mdANSI?style=flat-square)](https://github.com/justinhuangcode/mdANSI/stargazers)
[![Last Commit](https://img.shields.io/github/last-commit/justinhuangcode/mdANSI?style=flat-square)](https://github.com/justinhuangcode/mdANSI/commits/main)
[![Issues](https://img.shields.io/github/issues/justinhuangcode/mdANSI?style=flat-square)](https://github.com/justinhuangcode/mdANSI/issues)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-blue?style=flat-square)]()

极速 Markdown 转 ANSI 命令行工具，支持终端渲染、LLM 流式输出与语法高亮。 📝

---

## 为什么选择 MdANSI？

| 对比项 | mdANSI | [Markdansi](https://github.com/nicholasgasior/markdansi) | [mdcat](https://github.com/swsnr/mdcat) | [glow](https://github.com/charmbracelet/glow) |
|--------|--------|-----------|-------|------|
| 语言 | Rust | TypeScript | Rust | Go |
| 二进制大小 | ~4MB | N/A (需要 Node.js) | ~10MB | ~8MB |
| 运行时依赖 | 无 | Node.js 18+ | 无 | 无 |
| 语法高亮 | 内置 (syntect) | 外部 (Shiki) | 内置 (syntect) | 内置 (glamour) |
| 流式模式 | 支持 | 支持 | 不支持 | 不支持 |
| 自定义主题 | TOML 文件 | 仅代码 | 不支持 | Glamour JSON |
| GFM 表格 | 支持 | 支持 | 支持 | 支持 |
| 脚注 | 支持 | 不支持 | 支持 | 不支持 |
| 任务列表 | 支持 | 支持 | 支持 | 支持 |
| 数学公式 | LaTeX 透传 | 不支持 | 不支持 | 不支持 |
| OSC-8 超链接 | 支持 | 支持 | 支持 | 不支持 |
| 行号显示 | 支持 | 不支持 | 不支持 | 不支持 |
| 文本换行 | Unicode + CJK | 基础 | 基础 | 支持 |
| 启动时间 | ~1ms | ~100ms | ~5ms | ~3ms |

**mdANSI** 将 Rust 编译二进制的极致性能、完整的主题系统、以及 LLM 工作流所需的流式渲染能力融为一体。

---

## 功能特性

- **内置语法高亮** -- 通过 syntect 支持 200+ 种编程语言，零配置即用
- **完整 GFM 支持** -- 表格、任务列表、删除线、自动链接、脚注、数学公式（LaTeX 透传）
- **流式渲染模式** -- 为 LLM/AI 管道输出设计的增量渲染，多行结构自动缓冲
- **TOML 主题系统** -- 4 个内置主题 + 完全自定义的 `.toml` 主题文件
- **智能文本换行** -- Unicode 感知，CJK/Emoji 宽度正确，防孤字处理
- **OSC-8 超链接** -- 在支持的终端模拟器中可点击的链接
- **盒线代码块** -- 带语言标签和可选行号显示
- **自适应终端检测** -- 自动检测颜色等级、终端宽度和功能支持
- **单一静态二进制** -- ~4MB，无运行时依赖，瞬间启动
- **双模式 crate** -- 既可作为 CLI 工具使用，也可作为 Rust 库嵌入

---

## 安装

### 预构建二进制（推荐）

```bash
cargo binstall mdansi
```

### 从 crates.io 安装

```bash
cargo install mdansi
```

### 从源码编译

```bash
git clone https://github.com/justinhuangcode/mdANSI.git
cd mdANSI
cargo install --path .
```

### 验证安装

```bash
mdansi --version
```

---

## 快速开始

```bash
# 渲染 Markdown 文件
mdansi README.md

# 从 stdin 管道输入
cat CHANGELOG.md | mdansi

# 流式模式（用于 LLM 输出）
llm_command | mdansi --stream

# 使用自定义主题
mdansi --theme dracula doc.md

# 显示行号
mdansi -n README.md
```

---

## 命令选项

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `[FILE]` | 要渲染的 Markdown 文件（省略则读取 stdin） | -- |
| `-w, --width <N>` | 终端宽度覆盖 | 自动检测 |
| `-t, --theme <NAME>` | 颜色主题名称 | `default` |
| `--theme-file <PATH>` | 自定义 `.toml` 主题文件路径 | -- |
| `--no-wrap` | 禁用文本换行 | 关闭 |
| `--no-highlight` | 禁用语法高亮 | 关闭 |
| `-n, --line-numbers` | 在代码块中显示行号 | 关闭 |
| `--no-code-wrap` | 禁用代码块内换行 | 关闭 |
| `--table-border <S>` | 表格边框：`unicode` / `ascii` / `none` | `unicode` |
| `--no-truncate` | 禁用表格单元格截断 | 关闭 |
| `--color <MODE>` | 强制颜色：`always` / `never` / `auto` | `auto` |
| `-s, --stream` | 增量输入的流式模式 | 关闭 |
| `--plain` | 去除所有 ANSI 代码（纯文本输出） | 关闭 |
| `--list-themes` | 列出内置主题 | -- |
| `-h, --help` | 打印帮助信息 | -- |
| `-V, --version` | 打印版本号 | -- |

### 环境变量

| 变量 | 说明 |
|------|------|
| `MDANSI_WIDTH` | 覆盖终端宽度 |
| `MDANSI_THEME` | 默认主题名称 |
| `NO_COLOR` | 禁用所有颜色（[no-color.org](https://no-color.org)） |
| `FORCE_COLOR` | 强制颜色等级：`0`-`3` |

---

## 库用法

### 基础渲染

```rust
use mdansi::render_markdown;

let md = "# Hello\n\nThis is **bold** and *italic*.";
let ansi = render_markdown(md);
print!("{}", ansi);
```

### 自定义选项

```rust
use mdansi::{Renderer, RenderOptions, Theme, TerminalCaps};
use mdansi::theme;

let caps = TerminalCaps::detect();
let theme = theme::dracula_theme();
let options = RenderOptions {
    width: 100,
    line_numbers: true,
    ..RenderOptions::from_terminal(&caps)
};

let renderer = Renderer::new(theme, options);
let output = renderer.render("## Hello from mdANSI!");
print!("{}", output);
```

### 流式渲染（LLM 输出）

```rust
use mdansi::{StreamRenderer, RenderOptions, Theme};
use std::io;

let stdout = io::stdout().lock();
let mut stream = StreamRenderer::new(stdout, Theme::default(), RenderOptions::default());

// 当 LLM 产生输出时，逐块推送
stream.push("# 流式渲染\n").unwrap();
stream.push("这是").unwrap();
stream.push("**增量**").unwrap();
stream.push("输出。\n").unwrap();

// 流结束时刷新剩余缓冲区
stream.flush_remaining().unwrap();
```

---

## 主题

### 内置主题

| 主题 | 说明 |
|------|------|
| `default` | 为深色终端平衡的配色 |
| `solarized` | Solarized Dark 调色板 |
| `dracula` | Dracula 配色方案 |
| `monochrome` | 仅粗体/斜体/暗淡，无颜色 |

### 自定义 TOML 主题

创建一个 `.toml` 文件，可包含任意样式覆盖：

```toml
# my-theme.toml
[heading1]
fg = "#e06c75"
bold = true

[heading2]
fg = "#98c379"
bold = true

[inline_code]
fg = "#61afef"

[code_border]
fg = "#5c6370"
dim = true

[link_text]
fg = "#c678dd"
underline = true
```

```bash
mdansi --theme-file my-theme.toml README.md
```

**颜色格式：** 命名色 (`red`, `cyan`, `bright_blue`)、十六进制 (`#ff5733`)、256 色板索引 (`42`)。

---

## 工作原理

1. **解析** -- 通过 [comrak](https://github.com/kivikakk/comrak) 将 Markdown 输入解析为 AST（CommonMark + GFM 扩展）
2. **遍历** -- 深度优先遍历 AST，将每个节点转换为带样式的 ANSI 文本段
3. **高亮** -- 通过 [syntect](https://github.com/trishume/syntect) 对围栏代码块进行语法高亮
4. **布局** -- 表格使用 Unicode 感知的列宽度测量和盒线绘制边框进行布局
5. **换行** -- 长行在词边界处换行，正确处理 ANSI 转义序列和 CJK 字符宽度
6. **输出** -- 最终的 ANSI 字符串写入 stdout（库模式下返回 `String`）

在 **流式模式** 下，步骤 1-6 增量运行：单行内容立即输出，多行结构（代码块、表格）缓冲至完成后再输出。

---

## 架构

```
                  ┌──────────────┐
   Markdown ───>  │   parser.rs  │  comrak AST
                  └──────┬───────┘
                         │
                  ┌──────▼───────┐
                  │  render.rs   │  AST -> ANSI
                  └──┬───┬───┬──┘
                     │   │   │
          ┌──────────┘   │   └──────────┐
          ▼              ▼              ▼
   ┌────────────┐ ┌────────────┐ ┌────────────┐
   │ highlight  │ │  table.rs  │ │  wrap.rs   │
   │    .rs     │ │            │ │            │
   └────────────┘ └────────────┘ └────────────┘
     syntect        盒线绘制       Unicode 感知
     200+ 语言      列布局引擎     智能文本换行

   ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
   │  style.rs    │ │  theme.rs    │ │ hyperlink.rs │
   │  ANSI 编码   │ │  TOML 主题   │ │  OSC-8 链接  │
   └──────────────┘ └──────────────┘ └──────────────┘

   ┌──────────────┐ ┌──────────────┐
   │  stream.rs   │ │ terminal.rs  │
   │  LLM 流式    │ │  终端能力    │
   │  渲染器      │ │  检测        │
   └──────────────┘ └──────────────┘
```

---

## 项目结构

```
mdANSI/
├── src/
│   ├── lib.rs          # 公共 API 和重导出
│   ├── main.rs         # CLI 二进制入口
│   ├── cli.rs          # clap 参数定义
│   ├── parser.rs       # comrak Markdown 解析包装
│   ├── render.rs       # 核心 ANSI 渲染引擎
│   ├── stream.rs       # 流式渲染器（LLM 友好）
│   ├── style.rs        # ANSI 样式/颜色原语
│   ├── theme.rs        # 主题系统（TOML 支持）
│   ├── table.rs        # GFM 表格布局引擎
│   ├── highlight.rs    # syntect 语法高亮
│   ├── wrap.rs         # Unicode 感知文本换行
│   ├── hyperlink.rs    # OSC-8 终端超链接
│   ├── terminal.rs     # 终端能力检测
│   └── error.rs        # 错误类型
├── themes/
│   ├── default.toml    # 默认深色主题
│   ├── dracula.toml    # Dracula 主题
│   └── solarized.toml  # Solarized Dark 主题
├── tests/
│   ├── integration.rs  # 集成测试套件
│   └── fixtures/       # 测试固件
├── benches/
│   └── render.rs       # Criterion 性能基准
├── .github/
│   └── workflows/
│       └── ci.yml      # CI：检查、测试、clippy、格式、MSRV、审计
├── Cargo.toml
├── CHANGELOG.md
├── LICENSE-MIT
├── LICENSE-APACHE
└── README.md
```

---

## 性能基准

本地运行基准测试：

```bash
cargo bench
```

Apple M 系列硬件上的典型结果：

| 基准测试 | 耗时 |
|----------|------|
| 完整文档 + 语法高亮 | ~2ms |
| 完整文档，无高亮 | ~0.3ms |
| 纯文本输出 | ~0.2ms |

---

## 安全与环境

| 关注点 | 缓解措施 |
|--------|----------|
| 不受信任的 Markdown 输入 | comrak 沙盒化所有解析，不执行脚本 |
| 流缓冲区耗尽 | 10 MB 硬限制，自动刷新 |
| 主题文件加载 | 仅 TOML 反序列化，不执行代码 |
| 终端转义注入 | 所有用户内容通过 ANSI 样式层转义 |
| `NO_COLOR` 合规 | 完全支持 [no-color.org](https://no-color.org) 规范 |

---

## 故障排除

常见问题和解决方案请查看 [GitHub Issues](https://github.com/justinhuangcode/mdANSI/issues)。

| 问题 | 解决方案 |
|------|----------|
| 输出无颜色 | 检查 `NO_COLOR` 环境变量；使用 `--color always` 强制启用 |
| 表格列太窄 | 使用 `--width` 设置更宽的终端宽度，或使用 `--no-truncate` |
| 代码块未高亮 | 确保在开启围栏后指定了语言（如 ` ```rust `） |
| 流式输出乱码 | 确认终端支持 ANSI 转义序列 |

---

## 贡献

欢迎贡献！重大变更请先开 issue 讨论。

```bash
# 开发工作流
cargo build          # 构建
cargo test           # 运行所有测试（68 个测试）
cargo clippy         # 代码检查
cargo fmt --check    # 格式检查
cargo bench          # 性能基准
```

查看 [CHANGELOG.md](./CHANGELOG.md) 了解版本历史。

---

## 许可证

双重许可：[Apache License 2.0](LICENSE-APACHE) 或 [MIT License](LICENSE-MIT)，由您选择。

---

**mdANSI** 由 [Justin Huang](https://github.com/justinhuangcode) 维护。
