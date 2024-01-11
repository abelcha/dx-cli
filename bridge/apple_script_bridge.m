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

        // Specify the file or folder path
        // NSString *filePath = @"Macintosh HD:Users:abelchalier:dev:cover.ai";
        NSString *ar = [NSString stringWithUTF8String:path];
        NSURL *fileURL = [NSURL fileURLWithPath:ar];
        // NSString *hfsPath = CFBridgingRelease(CFURLCopyFileSystemPath((__bridge CFURLRef)(fileURL), kCFURLHFSPathStyle));

        // Get the information for the specified file or folder
        // FinderItem *item = [finder home][filePath];
        // SBElementArray *selection = [[finder selection] get];
        // FinderItem *item = [finder home];
        FinderItem *item = [[finder items] objectAtLocation: fileURL];
        return item.size;
}