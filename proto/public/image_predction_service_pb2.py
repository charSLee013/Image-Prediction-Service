# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: image_predction_service.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1dimage_predction_service.proto\x12\x10image_prediction\"B\n\x16ImagePredictionRequest\x12\r\n\x05image\x18\x01 \x01(\x0c\x12\r\n\x05model\x18\x02 \x01(\t\x12\n\n\x02id\x18\x03 \x01(\x05\"?\n\x13ImageVectorResponse\x12\x1c\n\x06vector\x18\x01 \x03(\x02R\x0cimage_vector\x12\n\n\x02id\x18\x02 \x01(\x05\"5\n\x05\x45rror\x12\x12\n\x04\x63ode\x18\x01 \x01(\x05R\x04\x63ode\x12\x18\n\x07message\x18\x02 \x01(\tR\x07message2q\n\x0fImagePrediction\x12^\n\x07Predict\x12(.image_prediction.ImagePredictionRequest\x1a%.image_prediction.ImageVectorResponse(\x01\x30\x01\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'image_predction_service_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_IMAGEPREDICTIONREQUEST']._serialized_start=51
  _globals['_IMAGEPREDICTIONREQUEST']._serialized_end=117
  _globals['_IMAGEVECTORRESPONSE']._serialized_start=119
  _globals['_IMAGEVECTORRESPONSE']._serialized_end=182
  _globals['_ERROR']._serialized_start=184
  _globals['_ERROR']._serialized_end=237
  _globals['_IMAGEPREDICTION']._serialized_start=239
  _globals['_IMAGEPREDICTION']._serialized_end=352
# @@protoc_insertion_point(module_scope)
