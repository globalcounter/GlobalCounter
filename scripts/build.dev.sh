#!/bin/sh

JNI_LIBS_DIR=./app/libs

# Build only for arm64-v8a and x86 targets for development purpose.
cargo build --target aarch64-linux-android
cargo build --target i686-linux-android

rm -rf $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR/arm64-v8a
mkdir $JNI_LIBS_DIR/x86

cp target/aarch64-linux-android/debug/libglobalcounter.so $JNI_LIBS_DIR/arm64-v8a/libglobalcounter.so
cp target/i686-linux-android/debug/libglobalcounter.so $JNI_LIBS_DIR/x86/libglobalcounter.so