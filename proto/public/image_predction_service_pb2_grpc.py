# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import image_predction_service_pb2 as image__predction__service__pb2


class ImagePredictionStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Predict = channel.stream_stream(
                '/image_prediction.ImagePrediction/Predict',
                request_serializer=image__predction__service__pb2.ImagePredictionRequest.SerializeToString,
                response_deserializer=image__predction__service__pb2.ImageVectorResponse.FromString,
                )


class ImagePredictionServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Predict(self, request_iterator, context):
        """Change the return type to stream ImageVectorResponse
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_ImagePredictionServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Predict': grpc.stream_stream_rpc_method_handler(
                    servicer.Predict,
                    request_deserializer=image__predction__service__pb2.ImagePredictionRequest.FromString,
                    response_serializer=image__predction__service__pb2.ImageVectorResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'image_prediction.ImagePrediction', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class ImagePrediction(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Predict(request_iterator,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.stream_stream(request_iterator, target, '/image_prediction.ImagePrediction/Predict',
            image__predction__service__pb2.ImagePredictionRequest.SerializeToString,
            image__predction__service__pb2.ImageVectorResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)