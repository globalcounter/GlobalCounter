#!/bin/sh

JNI_LIBS_DIR=./app/libs

cross build --target aarch64-linux-android --release
cross build --target armv7-linux-androideabi --release
cross build --target i686-linux-android --release
cross build --target x86_64-linux-android --release

rm -rf $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR
mkdir $JNI_LIBS_DIR/arm64-v8a
mkdir $JNI_LIBS_DIR/armeabi-v7a
mkdir $JNI_LIBS_DIR/x86
mkdir $JNI_LIBS_DIR/x86_64

cp target/aarch64-linux-android/release/libglobalcounter.so $JNI_LIBS_DIR/arm64-v8a/libglobalcounter.so
cp target/armv7-linux-androideabi/release/libglobalcounter.so $JNI_LIBS_DIR/armeabi-v7a/libglobalcounter.so
cp target/i686-linux-android/release/libglobalcounter.so $JNI_LIBS_DIR/x86/libglobalcounter.so
cp target/x86_64-linux-android/release/libglobalcounter.so $JNI_LIBS_DIR/x86_64/libglobalcounter.so