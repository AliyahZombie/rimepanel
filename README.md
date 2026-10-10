# RimePanel

轻量级 Minecraft 服务端 Web 管理面板，Rust 编写，单个可执行文件，开箱即用。

不需要预装 Node.js、Docker 或额外数据库服务。面板本身空载常驻内存仅约 5 MB，把机器资源尽可能留给 Minecraft 服务器。

![概览](docs/screenshots/dashboard.png)

## 核心特性

- **轻量低开销**：单二进制静态发布，空载常驻内存 ~5 MB，随开随用。
- **硬链接去重**：模组、服务端核心、整合包统一存入本地内容寻址库，实例内直接硬链接引用。多个实例共用同一批 mod 只占用一份磁盘空间；在实例内修改文件时自动脱链，不影响原件。
- **一键装服与 Java 托管**：
  - 支持 Vanilla、Paper、Purpur、Fabric、Quilt、Forge、NeoForge 等主流核心拉取与安装。
  - 根据所选 Minecraft 版本自动匹配并下载对应主版本的 Java（基于 Eclipse Temurin），也可手动指定或使用系统已有 Java。
- **整合包与 Mod 管理**：
  - 支持直接解析 Modrinth (`.mrpack`) 和 CurseForge 整合包。
  - 自动识别纯客户端 Mod 并移入 `mods-disabled/`，避免服务端启动崩溃。
  - 内置 Modrinth 资源搜索，按实例游戏版本和加载器过滤。
- **多端适配的 Web 界面**：
  - WebSocket 实时日志控制台、在线命令发送、性能监控（CPU、内存、在线人数）。
  - 在线文件管理（支持压缩解压，集成 Monaco 编辑器代码高亮编辑）。
  - 安全备份：触发 `save-off → save-all flush → save-on` 流程后打包为 `.tar.zst`，支持无人在线自动定时备份。
  - 界面自适应桌面、移动端与小屏设备。
- **国内网络适配**：下载源支持切换 BMCLAPI 镜像源；CurseForge 整合包支持通过 MCIM 镜像下载。

---

## 快速上手

### 直接运行

从 Release 页面下载对应架构的二进制（静态链接，无额外动态库依赖）：

```bash
chmod +x rimepanel-linux-x86_64
./rimepanel-linux-x86_64 --bind 0.0.0.0:8080 --data /srv/rimepanel
```

用浏览器访问 `http://<服务器IP>:8080`，首次打开按提示设置管理员密码即可。

### 命令行参数

| 参数 | 环境变量 | 默认值 | 说明 |
| --- | --- | --- | --- |
| `-b`, `--bind` | `RIMEPANEL_BIND` | `0.0.0.0:8080` | Web 监听地址 |
| `-d`, `--data` | `RIMEPANEL_DATA` | `./data` | 数据存放目录 |

> **注意**：为了让硬链接去重机制生效，数据目录内的 `instances/` 和 `library/` 需位于同一文件系统分区。若跨分区挂载，会自动降级为文件复制。

### 配置 systemd 服务

`/etc/systemd/system/rimepanel.service`：

```ini
[Unit]
Description=RimePanel Minecraft Panel
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=minecraft
ExecStart=/opt/rimepanel/rimepanel --bind 0.0.0.0:8080 --data /srv/rimepanel
Restart=on-failure
KillMode=mixed
TimeoutStopSec=90

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now rimepanel
```

---

## 数据目录结构

运行后 `--data` 指定路径下的布局如下：

```text
/srv/rimepanel
├── panel.db        # SQLite 数据库（面板设置、实例元数据、配方）
├── instances/      # 各 Minecraft 服务端根目录
├── library/        # 内容寻址资源库（objects/ 内容寻址文件，worlds/ 世界存档）
├── backups/        # 服务端备份文件（.tar.zst）
├── runtimes/       # 托管安装的 Java 运行时
├── cache/          # 版本清单与下载缓存
└── tmp/
```

---

## 界面预览

| 控制台 | 拼装服务器 |
| :---: | :---: |
| ![控制台](docs/screenshots/console.png) | ![拼装服务器](docs/screenshots/assemble.png) |

| Modrinth 安装与模组管理 | 文件管理 |
| :---: | :---: |
| ![Modrinth 安装](docs/screenshots/modrinth.png) | ![文件管理](docs/screenshots/files.png) |

| 备份管理 | 面板设置与 Java 运行时 |
| :---: | :---: |
| ![备份](docs/screenshots/backups.png) | ![设置](docs/screenshots/settings.png) |

<details>
<summary>移动端与小屏适配预览</summary>
<br>

<p align="center">
  <img src="docs/screenshots/m-dashboard.png" width="220" alt="手机 · 概览">
  &nbsp;&nbsp;
  <img src="docs/screenshots/m-console.png" width="220" alt="手机 · 控制台">
  &nbsp;&nbsp;
  <img src="docs/screenshots/w-home.png" width="160" alt="手表 · 实例列表">
  &nbsp;&nbsp;
  <img src="docs/screenshots/w-inst.png" width="160" alt="手表 · 实例详情">
</p>

</details>

---

## 从源码构建

需要 Rust stable 工具链：

```bash
cargo build --release
./target/release/rimepanel
```

前端为无构建步骤的原生 JavaScript（位于 `web/`），编译时通过 `rust-embed` 直接打包进二进制文件。

如需静态 musl 发布包，可通过 Zig 交叉编译：

```bash
scripts/build-musl.sh x86_64   # 或 aarch64，输出至 dist/
```

---

## 使用提示

- **反向代理**：控制台使用 WebSocket，任务进度使用 SSE。若使用 Nginx，需正确转发 `Upgrade` 请求头，并对 `/api/events` 关闭缓冲（`proxy_buffering off;`）。
- **Monaco 编辑器**：默认从 jsDelivr 加载。弱网环境下可在「设置 → 面板」替换为自建 CDN，加载失败会自动降级为纯文本编辑。
- **中国大陆网络**：建议在「设置 → 下载」中开启 BMCLAPI 镜像。
- **资源包分发**：如需玩家直连面板下载服务端资源包，需先在设置中配置面板公网访问地址。
- **登录鉴权**：目前为单管理员模式，会话保存在内存中，重启面板后需重新登录。

## 技术栈

Axum · Tokio · rusqlite · rust-embed · reqwest · argon2 · zstd
