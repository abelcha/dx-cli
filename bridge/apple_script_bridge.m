#import <Foundation/Foundation.h>
#import <ScriptingBridge/ScriptingBridge.h>
#import "Finder.h"


const char* executeAppleScript(const char *script) {
    NSString *scriptString = [NSString stringWithUTF8String:script];
    NSAppleScript *appleScript = [[NSAppleScript alloc] initWithSource:scriptString];
    NSDictionary *errorDict;
    NSAppleEventDescriptor *result = [appleScript executeAndReturnError:&errorDict];
    if (errorDict) {
        return strdup([[errorDict description] UTF8String]);
    }
    if (result) {
        return strdup([[result stringValue] UTF8String]);
    }
    return strdup("No result");
}

long long getFinderItemSize(const char *path) {
      FinderApplication *finder = [SBApplication applicationWithBundleIdentifier:@"com.apple.Finder"];
        NSString *ar = [NSString stringWithUTF8String:path];
        NSURL *fileURL = [NSURL fileURLWithPath:ar];
        FinderItem *item = [[finder items] objectAtLocation: fileURL];
        return item.size;
}
