
clang -c ./bridge/apple_script_bridge.m -o ./bridge/apple_script_bridge.o -fmodules -fobjc-arc -O3
ar rcs ./bridge/libapple_script_bridge.a ./bridge/apple_script_bridge.o 