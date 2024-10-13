# RCli

Rust 实现的 Cli 项目。

对行为进行编程 而不是对数据结构进行编程。

# CSV Convert

主要是 Rust 基础

# Gen Password

主要是 Rust 包管理

# Base64 Encode Decode

主要是学习单元测试

# Text Hash

继承、多态。 trait

# Http_server

从当前目录构建一个 web server

 - tokio -> 异步处理
 - hyper, reqwest, axum
 - tower, tower-http

todo:
 - [ ] 自动生成一个html 展示文件列表
 - [ ] html 可以增加搜索功能

# JWT Encode/Decode

 - Use Crate: jsonwebtoken
 - [ ] rcli jwt sign --sub acme --aud device --exp 14d
 - [ ] rcli jwt verify -t <token-value\>

# Self TODO

这里记录待办功能

 - [ ] cli JWT


# 优化相关

 - 重构
   - 已有代码进行可以考虑进行泛化
   - 代码精简、切片
 - 解决问题
   - 写 Unit test
   -
