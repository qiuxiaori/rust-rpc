# RPC 服务器

## 依赖

- [Rust](https://www.rust-lang.org/) 环境
- [Volo](https://www.cloudwego.io/zh/docs/volo/volo-grpc/getting-started/) 框架
- [gRPC](https://grpc.io/) 协议

## 结构

```
│ README.md
│ .gitignore
│ .env               // 环境配置文件
├─ idl               // .proto 文件目录
├─ cap-placement     // 电容排布核心包
├─ src
│  │  lib.rs
|  ├─ bin
│  └─ router
├─ volo-gen          // volo 生成
├─ target
```

## 功能

- health-check: 服务状态检测。
- cap-placement: BGA 电路板电容排布。

## 开发

1. 目录 `idl` 中新建 `.proto` 文件
2. `volo-gen/volo.yml` 文件中新增 `idls` 项
3. 执行 `cargo clean`
4. 执行 `cargo build`

## 命令

- 代码格式化: `cargo fmt`
- 启动服务端: `cargo run --bin server`
- 启动客户端: `cargo run --bin client`
