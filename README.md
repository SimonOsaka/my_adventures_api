# 我的奇遇记，后台

#### 项目结构
- handlers代表http服务内容
- domain代表域对象和逻辑定义
- db代表对数据操作

#### 运行命令
1. 开发运行命令`cargo run`
2. 打包`cargo build --release`

#### 第三方依赖
- warp
- sqlx
- chrono
- serde
- serde_json
- jsonwebtoken
- tokio
- dotenv
- pretty_env_logger
- log
- async-trait

#### linux
```shell
nohup ./rust-warp-sqlx-backend &
```

#### vscode插件
**项目根目录.vscode文件夹**
- extensions.json

**便捷命令执行**
- commandbar.json

**开发调试**
- 启动debug：launch.json
- 任务：tasks.json
