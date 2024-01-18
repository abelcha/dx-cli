#include <Carbon/Carbon.h>
#import <Foundation/Foundation.h>

long long getFinderFastFolderSize(const char *argPath) {
    NSString *file = [NSString stringWithUTF8String:argPath];
    CFURLRef url = (__bridge CFURLRef)[NSURL fileURLWithPath:file];
    Boolean isApplication = [[file pathExtension] isEqualToString:@"app"];
    Boolean isDirectory = CFURLHasDirectoryPath(url) && !isApplication;

    #pragma clang diagnostic push
    #pragma clang diagnostic ignored "-Wdeprecated-declarations"
    CFStringRef hfsPath = CFURLCopyFileSystemPath(url, kCFURLHFSPathStyle);
    #pragma clang diagnostic pop

    Size bufSize = (CFStringGetLength(hfsPath) + (isDirectory ? 1 : 0)) * sizeof(UniChar);
    UniCharPtr buf = malloc(bufSize);
    if (buf) {
        CFStringGetCharacters(hfsPath, CFRangeMake(0, bufSize / 2), buf);
        if (isDirectory) {
            buf[(bufSize - 1) / 2] = ':';
        }
        AEDesc nameDesc = { typeNull, nil };
        if (noErr == AECreateDesc(typeUnicodeText, buf, bufSize, &nameDesc)) {
            AEDesc aeDesc = { typeNull, nil };
            AEDesc containerDesc = { typeNull, nil };
            if (noErr == CreateObjSpecifier(isDirectory ? cFolder : cFile, &containerDesc, formName, &nameDesc, false, &aeDesc)) {
                AEBuildError aeBuildError;
                AppleEvent ae = { typeNull, nil };
                const OSType gFinderSignature = 'MACS';
                
                if (noErr == AEBuildAppleEvent(
                    kAECoreSuite,
                    kAEGetData,
                    typeApplSignature,
                    &gFinderSignature,
                    sizeof(OSType),
                    kAutoGenerateReturnID,
                    kAnyTransactionID,
                    &ae,
                    &aeBuildError,
                    "'----':obj {form:prop,want:type(prop),seld:type(ptsz),from:(@)}",
                    &aeDesc
                )) {
                    AppleEvent reply = { typeNull, nil };
                    if (noErr == AESend(&ae, &reply, kAEWaitReply, kAENormalPriority, kNoTimeOut, nil, nil)) {
                        NSAppleEventDescriptor *resultDescriptor = [[NSAppleEventDescriptor alloc] initWithAEDescNoCopy:&reply];
                        NSAppleEventDescriptor *item = [resultDescriptor descriptorAtIndex:1];
                        long long size = [item.stringValue longLongValue];
                        AEDisposeDesc(&reply);
                        return size;
                    }
                    AEDisposeDesc(&reply);
                    AEDisposeDesc(&ae);
                }
                AEDisposeDesc(&containerDesc);
            }
            AEDisposeDesc(&aeDesc);
            AEDisposeDesc(&nameDesc);
        }
        free(buf);
    }
    return -1;
}