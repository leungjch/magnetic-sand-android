```
cargo build --target aarch64-linux-android --release && cargo build --target armv7-linux-androideabi --release && cargo build --target i686-linux-android --release



ln -s /home/leungjch/AndroidStudioProjects/PhysicsFractals/cargo/target/aarch64-linux-android/release/libcargo.so jniLibs/arm64/libcargo.so &&  ln -s /home/leungjch/AndroidStudioProjects/PhysicsFractals/cargo/target/armv7-linux-androideabi/release/libcargo.so jniLibs/armeabi/libcargo.so && ln -s /home/leungjch/AndroidStudioProjects/PhysicsFractals/cargo/greetings/cargo/target/i686-linux-android/release/libcargo.so jniLibs/x86/libcargo.so
```
Run under `app/`
```
rm ./src/main/jniLibs/{arm64,arm64-v8a,x86,armeabi}/libcargo.so && \
ln -s /home/leungjch/AndroidStudioProjects/PhysicsFractals/cargo/target/aarch64-linux-android/release/libcargo.so ./src/main/jniLibs/arm64/libcargo.so &&
ln -s /home/leungjch/AndroidStudioProjects/PhysicsFractals/cargo/target/armv7-linux-androideabi/release/libcargo.so ./src/main/jniLibs/armeabi/libcargo.so &&  
ln -s /home/leungjch/AndroidStudioProjects/PhysicsFractals/cargo/greetings/cargo/target/i686-linux-android/release/libcargo.so ./src/main/jniLibs/x86/libcargo.so && ln -s /home/leungjch/AndroidStudioProjects/PhysicsFractals/cargo/target/aarch64-linux-android/release/libcargo.so ./src/main/jniLibs/arm64-v8a/libcargo.so
```
