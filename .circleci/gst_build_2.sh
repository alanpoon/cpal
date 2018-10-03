#!/bin/bash

set -eu

# Android Platform Version
ANDROID_PLATFORM="android-18"
# NDK home
NDK_HOME="/usr/local/android-ndk-r15b"
# Output Directory Path
OUTPUT_DIR="/usr/local/gstreamer/$ANDROID_PLATFORM"

# Android NDK standalone-toolchain Path
NDK_TOOLCHAIN_PATH="${OUTPUT_DIR}/toolchain"

# Android NDK standalone-toolchain(ARM)
ARM_TOOLCHAIN="arm-linux-androideabi-4.9"

# Android NDK standalone-toolchain(x86)
X86_TOOLCHAIN="x86-4.9"

# Android NDK standalone-toolchain Path(ARM)
ARM_TOOLCHAIN_PATH="${NDK_TOOLCHAIN_PATH}/arm"


# Android NDK standalone-toolchain Prefix(ARM)
ARM_TOOLCHAIN_PREFIX="${ARM_TOOLCHAIN_PATH}/bin/arm-linux-androideabi-"


################################################################################
#
################################################################################

# make Android NDK standalone-toolchain(ARM)
rm -rf "${ARM_TOOLCHAIN_PATH}"
"${NDK_HOME}/build/tools/make-standalone-toolchain.sh" --arch=arm --toolchain="${ARM_TOOLCHAIN}" --install-dir="${ARM_TOOLCHAIN_PATH}" --platform="${ANDROID_PLATFORM}" --verbose