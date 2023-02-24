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
    pushd lib
    echo "===================== lib ====================="
    ls
    popd
    echo "Coping libzfp.so into ${SDR_KIT_ROOT}/*/lib 4 times"
    cp ./lib/libzfp.so $SDR_KIT_ROOT/x86/lib
    cp ./lib/libzfp.so $SDR_KIT_ROOT/x86_64/lib
    cp ./lib/libzfp.so $SDR_KIT_ROOT/armeabi-v7a/lib
    cp ./lib/libzfp.so $SDR_KIT_ROOT/arm64-v8a/lib
    echo "Coping sucsessfull"
    cd ../../
}
build_zfp