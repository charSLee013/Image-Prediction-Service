import grpc
import image_predction_service_pb2
import image_predction_service_pb2_grpc
import time
import os

# Define a generator function to yield image requests
def generate_image_requests():
    # 读取图像数据
    with open('./images/image.png', 'rb') as f:
        image_data = f.read()
    # Specify the model name you want to use
    model_name = "illust2vec"
    # Yield the first image request
    for i in range(100):
        yield image_predction_service_pb2.ImagePredictionRequest(image=image_data, model=model_name,id = i)
        if i %4==0:
            time.sleep(3)

def run_client():
    # 创建 gRPC 连接
    channel = grpc.insecure_channel('127.0.0.1:1301')

    # 创建 ImagePredictionStub
    stub = image_predction_service_pb2_grpc.ImagePredictionStub(channel)

    # 创建 ImagePredictionRequest 对象
    request = image_predction_service_pb2.ImagePredictionRequest()

    try:
        # Call the service method with the request generator
        for entry_response in stub.Predict(generate_image_requests()):
            # Print the response
            print(len(entry_response.vector))

    except grpc.RpcError as e:
        print(f'Prediction failed: {e}')


if __name__ == '__main__':
    run_client()
