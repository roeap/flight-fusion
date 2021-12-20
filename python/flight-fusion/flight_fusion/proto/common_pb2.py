# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: common.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='common.proto',
  package='flight_fusion_ipc',
  syntax='proto3',
  serialized_options=None,
  create_key=_descriptor._internal_create_key,
  serialized_pb=b'\n\x0c\x63ommon.proto\x12\x11\x66light_fusion_ipc\"\"\n\x0e\x44\x65ltaReference\x12\x10\n\x08location\x18\x01 \x01(\t\" \n\x10ListingReference\x12\x0c\n\x04path\x18\x01 \x01(\t\"\x1d\n\rFileReference\x12\x0c\n\x04path\x18\x01 \x01(\t\"\xb8\x01\n\x0eTableReference\x12\x32\n\x05\x64\x65lta\x18\x01 \x01(\x0b\x32!.flight_fusion_ipc.DeltaReferenceH\x00\x12\x36\n\x07listing\x18\x02 \x01(\x0b\x32#.flight_fusion_ipc.ListingReferenceH\x00\x12\x31\n\x05\x66ilse\x18\x03 \x01(\x0b\x32 .flight_fusion_ipc.FileReferenceH\x00\x42\x07\n\x05table*^\n\rDatasetFormat\x12\x17\n\x13\x44\x41TASET_FORMAT_FILE\x10\x00\x12\x1a\n\x16\x44\x41TASET_FORMAT_DATASET\x10\x01\x12\x18\n\x14\x44\x41TASET_FORMAT_DELTA\x10\x02*T\n\x08SaveMode\x12\x19\n\x15SAVE_MODE_UNSPECIFIED\x10\x00\x12\x14\n\x10SAVE_MODE_APPEND\x10\x01\x12\x17\n\x13SAVE_MODE_OVERWRITE\x10\x02\x62\x06proto3'
)

_DATASETFORMAT = _descriptor.EnumDescriptor(
  name='DatasetFormat',
  full_name='flight_fusion_ipc.DatasetFormat',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='DATASET_FORMAT_FILE', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='DATASET_FORMAT_DATASET', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='DATASET_FORMAT_DELTA', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=323,
  serialized_end=417,
)
_sym_db.RegisterEnumDescriptor(_DATASETFORMAT)

DatasetFormat = enum_type_wrapper.EnumTypeWrapper(_DATASETFORMAT)
_SAVEMODE = _descriptor.EnumDescriptor(
  name='SaveMode',
  full_name='flight_fusion_ipc.SaveMode',
  filename=None,
  file=DESCRIPTOR,
  create_key=_descriptor._internal_create_key,
  values=[
    _descriptor.EnumValueDescriptor(
      name='SAVE_MODE_UNSPECIFIED', index=0, number=0,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='SAVE_MODE_APPEND', index=1, number=1,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
    _descriptor.EnumValueDescriptor(
      name='SAVE_MODE_OVERWRITE', index=2, number=2,
      serialized_options=None,
      type=None,
      create_key=_descriptor._internal_create_key),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=419,
  serialized_end=503,
)
_sym_db.RegisterEnumDescriptor(_SAVEMODE)

SaveMode = enum_type_wrapper.EnumTypeWrapper(_SAVEMODE)
DATASET_FORMAT_FILE = 0
DATASET_FORMAT_DATASET = 1
DATASET_FORMAT_DELTA = 2
SAVE_MODE_UNSPECIFIED = 0
SAVE_MODE_APPEND = 1
SAVE_MODE_OVERWRITE = 2



_DELTAREFERENCE = _descriptor.Descriptor(
  name='DeltaReference',
  full_name='flight_fusion_ipc.DeltaReference',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='location', full_name='flight_fusion_ipc.DeltaReference.location', index=0,
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
  serialized_start=35,
  serialized_end=69,
)


_LISTINGREFERENCE = _descriptor.Descriptor(
  name='ListingReference',
  full_name='flight_fusion_ipc.ListingReference',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='path', full_name='flight_fusion_ipc.ListingReference.path', index=0,
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
  serialized_start=71,
  serialized_end=103,
)


_FILEREFERENCE = _descriptor.Descriptor(
  name='FileReference',
  full_name='flight_fusion_ipc.FileReference',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='path', full_name='flight_fusion_ipc.FileReference.path', index=0,
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
  serialized_start=105,
  serialized_end=134,
)


_TABLEREFERENCE = _descriptor.Descriptor(
  name='TableReference',
  full_name='flight_fusion_ipc.TableReference',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  create_key=_descriptor._internal_create_key,
  fields=[
    _descriptor.FieldDescriptor(
      name='delta', full_name='flight_fusion_ipc.TableReference.delta', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='listing', full_name='flight_fusion_ipc.TableReference.listing', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR,  create_key=_descriptor._internal_create_key),
    _descriptor.FieldDescriptor(
      name='filse', full_name='flight_fusion_ipc.TableReference.filse', index=2,
      number=3, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
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
    _descriptor.OneofDescriptor(
      name='table', full_name='flight_fusion_ipc.TableReference.table',
      index=0, containing_type=None,
      create_key=_descriptor._internal_create_key,
    fields=[]),
  ],
  serialized_start=137,
  serialized_end=321,
)

_TABLEREFERENCE.fields_by_name['delta'].message_type = _DELTAREFERENCE
_TABLEREFERENCE.fields_by_name['listing'].message_type = _LISTINGREFERENCE
_TABLEREFERENCE.fields_by_name['filse'].message_type = _FILEREFERENCE
_TABLEREFERENCE.oneofs_by_name['table'].fields.append(
  _TABLEREFERENCE.fields_by_name['delta'])
_TABLEREFERENCE.fields_by_name['delta'].containing_oneof = _TABLEREFERENCE.oneofs_by_name['table']
_TABLEREFERENCE.oneofs_by_name['table'].fields.append(
  _TABLEREFERENCE.fields_by_name['listing'])
_TABLEREFERENCE.fields_by_name['listing'].containing_oneof = _TABLEREFERENCE.oneofs_by_name['table']
_TABLEREFERENCE.oneofs_by_name['table'].fields.append(
  _TABLEREFERENCE.fields_by_name['filse'])
_TABLEREFERENCE.fields_by_name['filse'].containing_oneof = _TABLEREFERENCE.oneofs_by_name['table']
DESCRIPTOR.message_types_by_name['DeltaReference'] = _DELTAREFERENCE
DESCRIPTOR.message_types_by_name['ListingReference'] = _LISTINGREFERENCE
DESCRIPTOR.message_types_by_name['FileReference'] = _FILEREFERENCE
DESCRIPTOR.message_types_by_name['TableReference'] = _TABLEREFERENCE
DESCRIPTOR.enum_types_by_name['DatasetFormat'] = _DATASETFORMAT
DESCRIPTOR.enum_types_by_name['SaveMode'] = _SAVEMODE
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

DeltaReference = _reflection.GeneratedProtocolMessageType('DeltaReference', (_message.Message,), {
  'DESCRIPTOR' : _DELTAREFERENCE,
  '__module__' : 'common_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.DeltaReference)
  })
_sym_db.RegisterMessage(DeltaReference)

ListingReference = _reflection.GeneratedProtocolMessageType('ListingReference', (_message.Message,), {
  'DESCRIPTOR' : _LISTINGREFERENCE,
  '__module__' : 'common_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.ListingReference)
  })
_sym_db.RegisterMessage(ListingReference)

FileReference = _reflection.GeneratedProtocolMessageType('FileReference', (_message.Message,), {
  'DESCRIPTOR' : _FILEREFERENCE,
  '__module__' : 'common_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.FileReference)
  })
_sym_db.RegisterMessage(FileReference)

TableReference = _reflection.GeneratedProtocolMessageType('TableReference', (_message.Message,), {
  'DESCRIPTOR' : _TABLEREFERENCE,
  '__module__' : 'common_pb2'
  # @@protoc_insertion_point(class_scope:flight_fusion_ipc.TableReference)
  })
_sym_db.RegisterMessage(TableReference)


# @@protoc_insertion_point(module_scope)