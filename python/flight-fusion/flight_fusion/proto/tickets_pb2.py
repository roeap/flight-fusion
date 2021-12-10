# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: tickets.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='tickets.proto',
  package='flight_fusion_ipc',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\rtickets.proto\x12\x11\x66light_fusion_ipc\"\x1a\n\tSqlTicket\x12\r\n\x05query\x18\x01 \x01(\t\"\x1a\n\tKqlTicket\x12\r\n\x05query\x18\x01 \x01(\t\"%\n\x15PutMemoryTableRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\"&\n\x16PutMemoryTableResponse\x12\x0c\n\x04name\x18\x01 \x01(\t\"%\n\x15PutRemoteTableRequest\x12\x0c\n\x04name\x18\x01 \x01(\t\"&\n\x16PutRemoteTableResponse\x12\x0c\n\x04name\x18\x01 \x01(\tb\x06proto3'
)




_SQLTICKET = _descriptor.Descriptor(
  name='SqlTicket',
  full_name='flight_fusion_ipc.SqlTicket',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='query', full_name='flight_fusion_ipc.SqlTicket.query', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=36,
  serialized_end=62,
)


_KQLTICKET = _descriptor.Descriptor(
  name='KqlTicket',
  full_name='flight_fusion_ipc.KqlTicket',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='query', full_name='flight_fusion_ipc.KqlTicket.query', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=64,
  serialized_end=90,
)


_PUTMEMORYTABLEREQUEST = _descriptor.Descriptor(
  name='PutMemoryTableRequest',
  full_name='flight_fusion_ipc.PutMemoryTableRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='flight_fusion_ipc.PutMemoryTableRequest.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=92,
  serialized_end=129,
)


_PUTMEMORYTABLERESPONSE = _descriptor.Descriptor(
  name='PutMemoryTableResponse',
  full_name='flight_fusion_ipc.PutMemoryTableResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='flight_fusion_ipc.PutMemoryTableResponse.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=131,
  serialized_end=169,
)


_PUTREMOTETABLEREQUEST = _descriptor.Descriptor(
  name='PutRemoteTableRequest',
  full_name='flight_fusion_ipc.PutRemoteTableRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='flight_fusion_ipc.PutRemoteTableRequest.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=171,
  serialized_end=208,
)


_PUTREMOTETABLERESPONSE = _descriptor.Descriptor(
  name='PutRemoteTableResponse',
  full_name='flight_fusion_ipc.PutRemoteTableResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='name', full_name='flight_fusion_ipc.PutRemoteTableResponse.name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=210,
  serialized_end=248,
)

DESCRIPTOR.message_types_by_name['SqlTicket'] = _SQLTICKET
DESCRIPTOR.message_types_by_name['KqlTicket'] = _KQLTICKET
DESCRIPTOR.message_types_by_name['PutMemoryTableRequest'] = _PUTMEMORYTABLEREQUEST
DESCRIPTOR.message_types_by_name['PutMemoryTableResponse'] = _PUTMEMORYTABLERESPONSE
DESCRIPTOR.message_types_by_name['PutRemoteTableRequest'] = _PUTREMOTETABLEREQUEST
DESCRIPTOR.message_types_by_name['PutRemoteTableResponse'] = _PUTREMOTETABLERESPONSE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

SqlTicket = _reflection.GeneratedProtocolMessageType('SqlTicket', (_message.Message,), {
  'DESCRIPTOR' : _SQLTICKET,
  '__module__' : 'tickets_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.SqlTicket)
  })
_sym_db.RegisterMessage(SqlTicket)

KqlTicket = _reflection.GeneratedProtocolMessageType('KqlTicket', (_message.Message,), {
  'DESCRIPTOR' : _KQLTICKET,
  '__module__' : 'tickets_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.KqlTicket)
  })
_sym_db.RegisterMessage(KqlTicket)

PutMemoryTableRequest = _reflection.GeneratedProtocolMessageType('PutMemoryTableRequest', (_message.Message,), {
  'DESCRIPTOR' : _PUTMEMORYTABLEREQUEST,
  '__module__' : 'tickets_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.PutMemoryTableRequest)
  })
_sym_db.RegisterMessage(PutMemoryTableRequest)

PutMemoryTableResponse = _reflection.GeneratedProtocolMessageType('PutMemoryTableResponse', (_message.Message,), {
  'DESCRIPTOR' : _PUTMEMORYTABLERESPONSE,
  '__module__' : 'tickets_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.PutMemoryTableResponse)
  })
_sym_db.RegisterMessage(PutMemoryTableResponse)

PutRemoteTableRequest = _reflection.GeneratedProtocolMessageType('PutRemoteTableRequest', (_message.Message,), {
  'DESCRIPTOR' : _PUTREMOTETABLEREQUEST,
  '__module__' : 'tickets_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.PutRemoteTableRequest)
  })
_sym_db.RegisterMessage(PutRemoteTableRequest)

PutRemoteTableResponse = _reflection.GeneratedProtocolMessageType('PutRemoteTableResponse', (_message.Message,), {
  'DESCRIPTOR' : _PUTREMOTETABLERESPONSE,
  '__module__' : 'tickets_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.PutRemoteTableResponse)
  })
_sym_db.RegisterMessage(PutRemoteTableResponse)


# @@protoc_insertion_point(module_scope)
