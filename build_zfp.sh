git clone https://github.com/LLNL/zfp.git 
# Build ZFP
build_zfp() { # [arch] [android_abi] [compiler_abi]
    echo "===================== ZFP ====================="
    ls
    cd zfp
    mkdir build  
    cd build  
    cmake ..  
    cmake --build . --config Release  
    cp ./lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/x86/lib
    cp ./lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/x86_64/lib
    cp ./lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/armeabi-v7a/lib
    cp ./lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/arm64-v8a/lib
    mv $SDR_KIT_ROOT/x86/lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/x86/lib/libzfp.so
    mv $SDR_KIT_ROOT/x86_64/lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/x86_64/lib/libzfp.so
    mv $SDR_KIT_ROOT/armeabi-v7a/lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/armeabi-v7a/lib/libzfp.so
    mv $SDR_KIT_ROOT/arm64-v8a/lib/libzfp.so.1.0.0 $SDR_KIT_ROOT/arm64-v8a/lib/libzfp.so
    cd ../../
}
build_zfp