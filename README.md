# protobuf_desc_demo

## parsing protobuf dynamically using protobuf FileDescriptor

this demo uses prost-reflect and FileDescriptor.desc of a given protobuf proto, 
in order to parse a buffer into a [vector's](https://github.com/vectordotdev/vector) VRL value.

test_protobuf.desc will be generated at build time by prost_build.