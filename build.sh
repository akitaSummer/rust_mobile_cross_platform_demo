cd android_code
cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o ./jniLibs build --release
JNI_LIBS="$(pwd)/jniLibs"
TEST_CLASS="$(pwd)/java/RustSM4.kt"

TARGET_ANDORID_LIBS="$(pwd)/../cross_platform_android/app/src/main/jniLibs"
TARGET_ANDORID_TEST_CLASS="$(pwd)/../cross_platform_android/app/src/main/java/com/cross/platform/RustSM4.kt"


cp -r $JNI_LIBS $TARGET_ANDORID_LIBS

cp -r $TEST_CLASS $TARGET_ANDORID_TEST_CLASS

cd ..
cd ios_code
sh ./build.sh