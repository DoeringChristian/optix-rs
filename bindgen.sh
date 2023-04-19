#!/bin/bash
set -exu

bindgen \
  --allowlist-type="^Optix.*" \
  --allowlist-type="RaygenRecord" \
  --allowlist-type="MissRecord" \
  --allowlist-type="HitgroupRecord" \
  --blocklist-type="OptixBuildInput" \
  --allowlist-function="optix.*" \
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
  --default-enum-style=rust \
  --with-derive-default \
  --with-derive-eq \
  --with-derive-hash \
  --with-derive-ord \
  --dynamic-loading OptixApi\
  wrapper.h -- -I$OPTIX_ROOT_DIR/include -I/opt/cuda/include\
  > src/optix.rs
