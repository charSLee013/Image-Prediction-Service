# 使用 clux/muslrust:stable 作为基础镜像,已经集成好musl环境了
FROM docker.io/clux/muslrust:stable as base

# 更新软件源并安装依赖包
RUN apt update -qq && apt install -y -qq protobuf-compiler build-essential

# 设置工作目录为 /usr/src/myapp
WORKDIR /usr/src/myapp

# 复制当前目录下的所有文件到工作目录中
COPY . .

# 编译项目为 release 模式
RUN cargo build --release 
RUN strip ./target/x86_64-unknown-linux-musl/release/image-prediction-service
RUN file ./target/x86_64-unknown-linux-musl/release/image-prediction-service

# 使用 alpine 作为最轻量的基础镜像
FROM alpine

# 从上一层镜像中复制二进制文件到当前镜像中
WORKDIR /app
COPY --from=0 /usr/src/myapp/target/x86_64-unknown-linux-musl/release/image-prediction-service ./image-prediction-service
COPY --from=0 /usr/src/myapp/config.yaml ./

# 声明程序会绑定 1301 端口
EXPOSE 1301

# 设置启动命令为运行二进制文件
CMD ["./image-prediction-service"]
