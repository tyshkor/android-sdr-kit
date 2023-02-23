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
    mv ./lib/libzfp.so.1.0.0 ../../SDRPlusPlus/misc_modules/scanner_cross_cross/libs/libzfp.so
    cd ../../
}
build_zfp