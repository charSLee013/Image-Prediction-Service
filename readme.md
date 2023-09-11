# 项目名称

## 项目简介
👋 这是一个使用rust语言和TensorFlow Serving构建的gRPC服务，它可以接收用户上传的图像，并返回图像的特征向量（feature vector），用于后续的图像检索（image retrieval）或其他应用。

## 功能特点
- 支持与 TensorFlow Serving 进行通信，发送图像预测请求。
- 支持解析 TensorFlow Serving 的响应数据，获取图像的特征向量。
- 支持 SIMD 指令集加速图像的 Base64 编码和解码。

## 系统要求
- CPU：支持 SIMD 指令集的处理器（如 SSE4.1、AVX2 等）。
- 内存：`128`MB 或更多。
- 核心数：`1` 核或更多。
- 操作系统：任意支持 `Rust` 和 `TensorFlow Serving` 的操作系统。

## 使用方法
1. 创建配置文件 `config.yaml`，根据需要添加模型信息。示例配置文件如下：

```yaml
models:
  - name: model1
    version: 1
    input_name: input_tensor_name
  - name: model2
    version: 2
    input_name: input_tensor_name
```

2. 如果您不知道模型的 `input_name` 是什么，可以使用以下命令来查看模型的输入层详细结构：

```shell
saved_model_cli show --dir <model path> --tag_set serve --signature_def serving_default
```

显示内容大概如下：

```text
The given SavedModel SignatureDef contains the following input(s):
  inputs['b64_input_bytes'] tensor_info:
      dtype: DT_STRING
      shape: (-1)
      name: serving_default_b64_input_bytes:0
The given SavedModel SignatureDef contains the following output(s):
  outputs['resnet_custom_v3'] tensor_info:
      dtype: DT_FLOAT
      shape: (-1, 9176)
      name: StatefulPartitionedCall:0
Method name is: tensorflow/serving/predict
```

通过查看模型的输入层详细结构，您可以获得正确的 `input_name` 值，并将其添加到配置文件中。

3. 使用以下命令行参数来运行程序：

```shell
cargo run -- --config=config.yaml --addr=0.0.0.0:1301 --tensorflow_api_addr=http://localhost:8501/v1
```

### 命令行参数说明：

- `--config`：指定配置文件的路径，默认为 `config.yaml`。
- `--addr`：指定要绑定的 IP 地址和端口，默认为 `0.0.0.0:1301`。
- `--tensorflow_api_addr`：指定 TensorFlow Serving 的 RESTful API 地址，默认为 `http://localhost:8501/v1`。

确保每个模型的配置正确，并将其添加到配置文件中。

### 打印调试信息

可以在运行之前设置`RUST_LOG`环境变量来打印调试信息等级
```
export RUST_LOG=DEBUG
```

## 测试
你可以使用项目中的 `proto/public/test_client.py` 文件进行测试

1. 首先，请确保您已经安装了相关的库，您可以使用以下命令来安装所需的库：
```shell
python3 -m pip install grpcio-tools grpcio
```

2. 确保服务端已经启动并正在监听相应的地址和端口。

3. 打开终端，并进入项目的根目录。

4. 运行以下命令来执行测试客户端：
```shell
python3 proto/public/test_client.py
```

5. 测试客户端将会连接到服务端，并发送一些示例请求，然后打印响应结果。

6. 如果服务端的环境变量

## 作者
编写者：charslee013

## 贡献
欢迎对该项目进行贡献。如果有任何问题、建议或意见，请随时联系我们。

## 许可证
该项目基于 Apache 2.0 许可证。请查阅 [LICENSE](LICENSE) 文件以获取更多信息。

感谢您的使用和贡献！🎉