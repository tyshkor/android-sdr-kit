set -e

# Settings
export ANDROID_INSTALLER=commandlinetools-linux-8512546_latest.zip
export ANDROID_API_LEVEL=28
export ANDROID_NDK_VERSION=25.1.8937393
export ANDROID_CMAKE_VERSION=3.18.1
export GRADLE_VERSION=7.3.3
export SDR_KIT_BUILD=/root/sdr-kit-build
export SDR_KIT_ROOT=/sdr-kit
export MAKEOPTS=-j$(nproc)

apt-get update -y
sh -c 'echo "deb http://deb.debian.org/debian stretch main\ndeb-src http://deb.debian.org/debian stretch main" >> /etc/apt/sources.list'
apt install -y debian-archive-keyring
apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 04EE7237B7D453EC 648ACFD622F3D138 0E98404D386FA1D9 EF0F382A1A7B6500

# Install dependencies
apt update -y
apt install -y libcrypt-dev
apt-get update -y
apt-get install -y gcc-arm-linux-androideabi
apt install -y wget git p7zip-full default-jdk build-essential cmake patchelf python3-mako build-essential pkg-config libssl-dev clang libclang-dev libc6-dev gcc-multilib
apt install -y python3-distutils
apt install -y python3-apt

# Install the Android tools
mkdir Android
wget https://dl.google.com/android/repository/$ANDROID_INSTALLER
7z x $ANDROID_INSTALLER
mv cmdline-tools/ Android/
export ANDROID_HOME=/root/Android
export ANDROID_SDK_ROOT=/root/Android

# Install the SDK and NDK
yes | $ANDROID_HOME/cmdline-tools/bin/sdkmanager --sdk_root=$ANDROID_HOME --install "platforms;android-$ANDROID_API_LEVEL" "ndk;$ANDROID_NDK_VERSION" "cmake;$ANDROID_CMAKE_VERSION"
export ANDROID_NDK_TOOLCHAIN=$ANDROID_HOME/ndk/$ANDROID_NDK_VERSION/toolchains/llvm/prebuilt/linux-x86_64/bin
export ANDROID_NDK_CMAKE=$ANDROID_HOME/ndk/$ANDROID_NDK_VERSION/build/cmake/android.toolchain.cmake

# Install gradle
mkdir -p /opt/gradle/
wget https://services.gradle.org/distributions/gradle-$GRADLE_VERSION-bin.zip
7z x gradle-$GRADLE_VERSION-bin.zip
cp -r gradle-$GRADLE_VERSION /opt/gradle/
echo "export GRADLE_HOME=/opt/gradle/gradle-$GRADLE_VERSION" >> /etc/profile
echo "export PATH=\$GRADLE_HOME/bin:\$PATH" >> /etc/profile

# Create directories
mkdir -p $SDR_KIT_BUILD
mkdir -p $SDR_KIT_ROOT
echo "export SDR_KIT_ROOT=$SDR_KIT_ROOT" >> /etc/profile

# Do build
cd $SDR_KIT_BUILD
chmod +x /root/build.sh
chmod +x /root/package.sh
/root/build.sh

# Setup environment variables at launch
echo "export ANDROID_SDK_ROOT=$ANDROID_SDK_ROOT" >> /etc/profile