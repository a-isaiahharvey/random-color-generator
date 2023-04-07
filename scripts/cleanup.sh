#!/bin/sh

# Evironment Variables
JNI_LIBS=android/app/src/main/jniLibs

# Clean library files from Android project
rm -rf $JNI_LIBS

# Clean library folder
cargo clean

# Clean web folder
cd web
trunk clean