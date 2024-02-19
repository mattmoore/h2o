#!/usr/bin/bash

export BUILD_DIR=target/release

cargo clean \
  && if [ -d $BUILD_DIR/pkg ]; then rm -rf $BUILD_DIR/pkg; fi \
  && cargo build -r \
  && mkdir $BUILD_DIR/pkg \
  && cd $BUILD_DIR \
  && cp \
    h2o \
    libh2o.rlib \
    libh2o.a \
    libh2o.so \
      pkg/ \
  && cp \
    build/cli-ca1562d982faecec/out/_h2o \
    build/cli-ca1562d982faecec/out/h2o.bash \
      pkg/ \
  && cd pkg \
  && tar cvJf ../h2o-0.1.0.tar.xz * \
  && cd ../..
