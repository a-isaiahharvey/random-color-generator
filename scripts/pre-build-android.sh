#!/bin/sh
JNI_LIBS=android/app/src/main/jniLibs

cargo ndk -t armeabi-v7a -t arm64-v8a -o $JNI_LIBS build --release

rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a
mkdir $JNI_LIBS/x86

cp target/aarch64-linux-android/release/librandom_color_generator.so $JNI_LIBS/arm64-v8a/librandom_color_generator.so
cp target/armv7-linux-androideabi/release/librandom_color_generator.so $JNI_LIBS/armeabi-v7a/librandom_color_generator.so
