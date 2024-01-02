#import <Foundation/Foundation.h>

const char* executeAppleScript(const char *script) {
    NSString *scriptString = [NSString stringWithUTF8String:script];
    NSAppleScript *appleScript = [[NSAppleScript alloc] initWithSource:scriptString];
    NSDictionary *errorDict;
    NSAppleEventDescriptor *result = [appleScript executeAndReturnError:&errorDict];
    if (errorDict) {
        NSLog(@"AppleScript Error: %@", errorDict);
        return strdup([[errorDict description] UTF8String]);
    }
    if (result) {
        return strdup([[result stringValue] UTF8String]);
    }
    return strdup("No result");
}

