#!/bin/bash
ROOT=${PWD}
cd app/src/main/

rm -rf jniLibs
mkdir -p jniLibs/armeabi

ln -s ${ROOT}/target/arm-linux-androideabi/debug/libservo_media_android.so jniLibs/armeabi/libmain.so
ln -s ${ROOT}/gstreamer/armeabi-v7a/gst-build-armeabi-v7a/libgstreamer_android.so jniLibs/armeabi/libgstreamer_android.so
ln -s /root/project/assets assets
cd ${ROOT}/app
./gradlew installDebug || return 1