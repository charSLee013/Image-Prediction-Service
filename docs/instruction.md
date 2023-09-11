# Image Prediction Service

👋 欢迎来到Image Prediction Service项目，这是一个使用rust语言和TensorFlow Serving构建的gRPC服务，它可以接收用户上传的图像，并返回图像的特征向量（feature vector），用于后续的图像检索（image retrieval）或其他应用。

## 项目目的

本项目旨在展示如何使用rust语言和TensorFlow Serving搭建一个高性能、高并发、高可用的gRPC服务，以及如何使用proto文件定义数据结构和服务接口，以实现跨语言和跨平台的通信。

## 项目功能

本项目提供了以下功能：

- 从配置文件中读取模型信息，包括模型名称、版本、输入名称等。
- 从命令行参数或环境变量中读取服务地址和TensorFlow Serving地址。
- 启动一个gRPC服务器，监听用户请求，并将其转发给TensorFlow Serving模型。
- 接收用户上传的图像数据，并将其编码为Base64格式，以便发送给TensorFlow Serving模型。
- 从TensorFlow Serving模型获取图像特征向量，并将其作为响应返回给用户。
- 使用流式（stream）通信方式，提高性能和效率。
- 使用日志系统记录服务运行情况，并支持不同级别（level）和目标（target）的日志输出。
- 使用错误处理库处理可能发生的错误，并返回合适的错误信息和状态码。

## 项目结构

本项目包含以下几个部分：

- `build.rs`：这是一个构建脚本，用于在编译前执行一些操作。在本项目中，它用于生成proto文件。
- `Cargo.lock`：这是一个锁定文件，用于记录项目的依赖关系和版本。它可以确保项目在不同的环境中使用相同的依赖版本。
- `Cargo.toml`：这是一个配置文件，用于声明项目的元数据、依赖、特性和构建设置。它是Cargo工具的核心。
- `config.yaml`：这是一个YAML格式的配置文件，用于存储项目的一些自定义参数，例如模型名称、端口号、日志级别等。
- `deps_install.sh`：这是一个shell脚本，用于安装项目所需的一些外部依赖，例如TensorFlow Serving。
- `images`：这是一个存放图像文件的目录，包含两个图像文件：`image.jpg`和`image.png`。这些图像可以用于测试项目的功能和性能。
- `proto`：这是一个存放proto文件和相关代码的目录，包含一个子目录`public`。proto文件是一种用于定义数据结构和服务接口的语言，常用于跨语言和跨平台的通信。
  - `public`：这是一个存放公共proto文件和相关代码的子目录，包含以下几个文件：
    - `image_predction_service_pb2_grpc.py`：这是一个由grpc工具根据proto文件生成的Python代码文件，用于实现服务端和客户端的通信逻辑。
    - `image_predction_service_pb2.py`：这是一个由protoc工具根据proto文件生成的Python代码文件，用于定义数据结构和服务接口。
    - `image_predction_service_pb2.pyi`：这是一个由mypy-protobuf工具根据proto文件生成的Python类型提示文件，用于提供静态类型检查。
    - `image_predction_service.proto`：这是一个自定义的proto文件，用于定义图像预测服务的数据结构和接口。
    - `__pycache__`：这是一个存放Python编译后字节码文件的目录，用于提高程序运行速度。
    - `test_client.py`：这是一个自定义的Python代码文件，用于测试图像预测服务的功能和性能。
- `src`：这是一个存放rust源代码文件的目录，包含以下几个文件：
  - `config.rs`：这是一个自定义的rust代码文件，用于读取和解析配置文件中的参数，并提供给其他模块使用。
  - `input.rs`：这是一个自定义的rust代码文件，用于处理用户输入参数并按照特定方式启动。
  - `lib.rs`：这是一个rust库模块，用于声明项目中使用到的一些公共函数、类型或特性。
  - `logger.rs`：这是一个自定义的rust代码文件，用于初始化日志系统，并提供一些日志记录函数。
  - `main.rs`：这是一个rust可执行模块，用于启动项目并调用其他模块的功能。
  - `pb.rs`：这是一个自定义的rust代码文件，用于导入proto文件生成的rust代码，并提供一些辅助函数。
  - `proto-gen`：这是一个存放proto文件生成的rust代码的目录，包含一个文件：
    - `image_prediction.rs`：这是一个由prost工具根据proto文件生成的rust代码文件，用于定义数据结构和服务接口。
  - `service.rs`：这是一个自定义的rust代码文件，用于实现图像预测服务的业务逻辑，包括接收用户请求、调用TensorFlow Serving模型、处理模型输出、发送用户响应等。
  - `tf_serving`：这是一个存放与TensorFlow Serving相关的rust代码的目录，包含以下几个文件：
    - `model_status.rs`：这是一个自定义的rust代码文件，用于定义和实现与TensorFlow Serving模型状态服务相关的数据结构和接口。
    - `mod.rs`：这是一个rust模块声明文件，用于将当前目录下的其他文件作为子模块导出。
    - `predict_service.rs`：这是一个自定义的rust代码文件，用于定义和实现与TensorFlow Serving预测服务相关的数据结构和接口。

## 项目使用

本项目使用Cargo工具进行编译、运行和测试。您可以在终端中输入以下命令来执行相应的操作：

- 编译项目：

```bash
cargo build
```

- 运行项目：

```bash
cargo run -- [options]
```

其中[options]可以指定以下参数：

- `-c, --config <config>`: 指定配置文件的路径，默认为"config.yaml"。
- `-a, --addr <addr>`: 指定服务地址和端口，默认为"0.0.0.0:1301"。也可以通过设置环境变量"ADDR"来指定。
- `--tensorflow_api_addr <tensorflow_api_addr>`: 指定TensorFlow Serving RESTful API地址，默认为"http://localhost:8501/v1"。

- 测试项目：

```bash
cargo test
```