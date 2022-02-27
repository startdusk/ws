# Rust 开发 Web 项目演示代码

参考视频地址: [Rust Web 全栈开发教程](https://www.bilibili.com/video/BV1RP4y1G7KF?p=17)

## 项目结构

<img src="./docs/power.png" style="max-width:100%;" />

## 运行开发

### webservice

1.修改 `.env.example` 为 `.env` 并设置为你的 PostgreSQL 数据库连接和 `webapp` 的端口

2.初始化数据库, 初始化 SQL 文件, [点击这里](./webservice/docs/)

3.根目录运行:

```bash
$ make run
```

### webapp

后端渲染，没啥可说的，上面的 `make run` 一并和 `webservice` 运行了

### wasm-client

注意：这里用 Webassembly 开发绕了一个大圈, 主要是为了学习 Webassembly 在 Web 中的使用, 并不是所有的 Rust crate 都能在 Webassembly 里使用, 只有一小部分能使用

1.编译

```
$ cd wasm-client
$ wasm-pack build
```

2.让前端加载编译好的 wasm

```
$ cd www
$ npm install
```

3.运行 www 前端

```
$ npm run start
```

## 编译发布

根目录下

1.编译 `webservice` 和 `webapp`

```bash
$ cargo build --bin teacher-service --release
$ cargo build --bin svr --release
```

然后去根目录下的 target 目录取它的二进制文件即可放在服务器运行, 注意执行前要把 `.env` 文件里面的环境变量设置上

2.编译 `wasm-client`

```bash
$ wasm-pack build --release
$ cd wwww
$ npm install
$ npm build
```

然后取 `dist` 目录去服务器
编译好的 `dist` 本身不能运行, 它需要一个 http 服务器, 可以在服务器安装 `http-server`

```bash
$ npm install -g http-server
$ http-server ./dist -p 8081 # 可以指定你想要的端口
```
