#!/bin/bash
set -exu

bindgen \
  --allowlist-type="^Optix.*" \
  --allowlist-type="RaygenRecord" \
  --allowlist-type="MissRecord" \
  --allowlist-type="HitgroupRecord" \
  --allowlist-function="optixQueryFunctionTable" \
  --allowlist-var="OptixSbtRecordHeaderSize" \
  --allowlist-var="OptixSbtRecordAlignment" \
  --allowlist-var="OptixAccelBufferByteAlignment" \
  --allowlist-var="OptixInstanceByteAlignment" \
  --allowlist-var="OptixAabbBufferByteAlignment" \
  --allowlist-var="OptixGeometryTransformByteAlignment" \
  --allowlist-var="OptixTransformByteAlignment" \
  --allowlist-var="OptixVersion" \
  --allowlist-var="OptixBuildInputSize" \
  --allowlist-var="OptixShaderBindingTableSize" \
  --allowlist-var="OPTIX_.*" \
  --allowlist-var="^Optix.*" \
  --allowlist-type="^Optix.*" \
  --blocklist-type="^CU.*" \
  --default-enum-style=rust \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  --with-derive-custom-struct="OptixBuildInputTriangleArray=Debug" \
  --with-derive-custom-struct="OptixShaderBindingTable=Debug" \
  --vtable-generation \
  wrapper.h -- -I$OPTIX_ROOT_DIR/include -I/opt/cuda/include\
  > src/autogen_optix.rs
