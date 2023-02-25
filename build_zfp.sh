git clone https://github.com/LLNL/zfp.git 
# Build ZFP
build_zfp() { # [arch] [android_abi] [compiler_abi]
    echo "===================== ZFP ====================="
    cd zfp
    mkdir build  
    cd build  
    cmake ..  
    cmake --build . --config Release
    echo "Coping libzfp.so into /root/SDRPlusPlus/root/res/bandplans"
    mv ./lib/libzfp.so.1.0.0 /root/SDRPlusPlus/root/res/bandplans/libzfp.so
    echo "Coping sucsessfull"
    cd ../../
}
build_zfp
rm -r zfp