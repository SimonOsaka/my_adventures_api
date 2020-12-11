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

#### postgresql-13
```postgresql
CREATE SEQUENCE IF NOT EXISTS my_adventures_id_seq;
CREATE TABLE "public"."my_adventures" (
  "id" int8 NOT NULL DEFAULT nextval('my_adventures_id_seq'::regclass),
  "title" varchar(40) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "created_at" timestamp(0) NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "is_deleted" int2 NOT NULL DEFAULT 0,
  "image_url" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "item_type" int2 NOT NULL DEFAULT 1,
  "link" varchar(255) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "source" int2 NOT NULL DEFAULT 0,
  "journey_destiny" varchar(12) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "script_content" varchar(140) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "play_list" varchar(16) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "douban_id" int8 NOT NULL DEFAULT 0,
  "douban_rank" int2 NOT NULL DEFAULT 0,
  "address" varchar(100) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "shop_name" varchar(20) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "province" varchar(7) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "city" varchar(10) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  "district" varchar(10) COLLATE "pg_catalog"."default" NOT NULL DEFAULT ''::character varying,
  CONSTRAINT "my_adventures_pkey" PRIMARY KEY ("id")
);
```

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
