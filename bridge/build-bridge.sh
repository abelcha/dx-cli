
clang -c apple_script_bridge.m -o apple_script_bridge.o -fmodules -fobjc-arc -framework Foundation
ar rcs libapple_script_bridge.a apple_script_bridge.o 