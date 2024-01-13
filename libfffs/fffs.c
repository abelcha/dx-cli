#include <stdio.h>
#include <unistd.h>
#include <errno.h>
#include <sys/stat.h>
#include <Carbon/Carbon.h>
#include <string.h>

static OSErr PrintOSXComment(char * path);
pascal OSErr FFFS_FEGetSize(const FSRef * pFSRefPtr, char * pStr, const AEIdleUPP pIdleProcUPP);
pascal void FFFS_AEDisposeDesc(AEDesc * desc);
pascal void FFFS_AENullDesc(AEDesc * desc);
pascal OSStatus FFFS_AEOCreateObjSpecifierFromFSRef(const FSRefPtr pFSRefPtr, AEDesc * pObjSpecifier);
pascal OSStatus FFFS_AEOCreateObjSpecifierFromCFURLRef(const CFURLRef pCFURLRef, AEDesc * pObjSpecifier);
pascal OSStatus FFFS_AESendEventReturnAEDesc(const AEIdleUPP pIdleProcUPP, const AppleEvent * pAppleEvent, const DescType pDescType, AEDesc * pAEDesc);
pascal OSStatus FFFS_AEGetHandlerError(const AppleEvent * pAEReply);
pascal OSStatus FFFS_AESendEventReturnData(const AEIdleUPP pIdleProcUPP, const AppleEvent * pAppleEvent, DescType pDesiredType, DescType * pActualType, void * pDataPtr, Size pMaximumSize, Size * pActualSize);
Boolean MyAEIdleCallback(EventRecord * theEvent, SInt32 * sleepTime, RgnHandle * mouseRgn);

#define FFFS_Assert(x)(true)
#define FFFS_AssertQ(x)

#include <time.h>


// #define MEASURE_EXECUTION_TIME_INT(function, message, result) do { \
//     clock_t start = clock(); \
//     (result) = (function); \
//     clock_t end = clock(); \
//     double cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC; \
//     printf("%s: %f seconds\n", (message), cpu_time_used); \
// } while (0)

#define MEASURE_EXECUTION_TIME_INT(function, message, result) do { \
    (result) = (function); \
} while (0)



// The Mac Four-Character Application Signature for the Finder
static
const OSType gFinderSignature = 'MACS';

int main(int argc,
  const char * argv[]) {
  OSErr err = noErr;
  int rc;
  int optch;
  char * path;
  int type;
  //path to file passed as argument
  path = (char * ) argv[optind];
  if (path == NULL) {
    exit(0);
  }
	int canAccess;
	MEASURE_EXECUTION_TIME_INT(access(path, R_OK | F_OK), "canAccess", canAccess);
  if (canAccess == -1) {
    perror(path);
    exit(1);
  }

  MEASURE_EXECUTION_TIME_INT(PrintOSXComment( path), "EXECALLLL", err);
  exit(err);

  return err;
}


static OSErr PrintOSXComment(char * path) {
  OSErr err = noErr;
	FSRef fileRef;
  FSSpec fileSpec;
  AEIdleUPP inIdleProc = NewAEIdleUPP( & MyAEIdleCallback);
	char pStr[256];
	memset(pStr, 0, sizeof(pStr));
  //retrieve filespec from file ref
  // Get file ref to the file or folder pointed to by the path
  MEASURE_EXECUTION_TIME_INT(FSPathMakeRef((unsigned char * ) path, & fileRef, NULL), "FSPathRef", err);;
  if (err != noErr) {
    fprintf(stderr, "FSPathMakeRef(): Error %d returned when getting file reference for %s\n", err, path);
    exit(1);
  }

  err = FFFS_FEGetSize(&fileRef, pStr, inIdleProc);
  if (err) {
    fprintf(stderr, "Error %d getting comment\n", err);
    if (err == -600)
      fprintf(stderr, "Finder is not running\n", err); //requires Finder to be running
    return err;
  }
  return noErr;
}

#pragma mark -

  Boolean MyAEIdleCallback(
    EventRecord * theEvent,
    SInt32 * sleepTime,
    RgnHandle * mouseRgn) {

    return 0;
  }

pascal OSErr FFFS_FEGetSize(const FSRef * pFSRefPtr, char *pStr, const AEIdleUPP pIdleProcUPP) {
  AppleEvent tAppleEvent = {
    typeNull,
    NULL
  }; //  If you always init AEDescs, it's always safe to dispose of them.
  AEDesc tAEDesc = {
    typeNull,
    NULL
  };
	SInt64 *pFont;
  OSErr anErr = noErr;
	DescType actualType;
  Size actualSize;
  OSStatus anError;
	actualSize = 16;

  if (NULL == pIdleProcUPP) {
		 // the idle proc is required
    fprintf(stderr, "No proc pointer\n");
    return paramErr;
  }
	
	
  MEASURE_EXECUTION_TIME_INT(FFFS_AEOCreateObjSpecifierFromFSRef(pFSRefPtr, & tAEDesc), "CreatEObj", anErr);
	

  if (anErr) {
    fprintf(stderr, "Error creating objspecifier from fsspec\n");
    return paramErr;
  }
  if (noErr == anErr) {
    AEBuildError tAEBuildError;

    MEASURE_EXECUTION_TIME_INT(AEBuildAppleEvent(
      kAECoreSuite,
			kAEGetData,
      typeApplSignature,
			&gFinderSignature,
			sizeof(OSType),
      kAutoGenerateReturnID,
			kAnyTransactionID,
			&tAppleEvent,
			&tAEBuildError,
      "'----':obj {form:prop,want:type(prop),seld:type(ptsz),from:(@)}",
			&tAEDesc
		), "buildEvnt", anErr);

    // always dispose of AEDescs when you are finished with them
    (void) FFFS_AEDisposeDesc( & tAEDesc);

    if (noErr == anErr) {
			MEASURE_EXECUTION_TIME_INT(FFFS_AESendEventReturnData(pIdleProcUPP, &tAppleEvent, typeChar, &actualType, pStr, 255, & actualSize), "sendEvent", anErr);
			printf("response '%s' \n", pStr);
      if (anErr) {
        fprintf(stderr, "Error sending event to get pascal string\n");
      }
      // always dispose of AEDescs when you are finished with them
      (void) FFFS_AEDisposeDesc( & tAppleEvent);
    } else {
      fprintf(stderr, "Error building Apple Event\n");
    }
  }
  return anErr;
} // end FFFS_FEGetSize

/********************************************************************************
  Send an Apple event to the Finder to get the finder comment of the item 
  specified by the FSRefPtr.

  pFSRefPtr    ==>    The item to get the file kind of.
  pCommentStr    ==>    A string into which the finder comment will be returned.
  pIdleProcUPP  ==>    A UPP for an idle function (required)
  
  See note about idle functions above.
*/

//*******************************************************************************
// Disposes of desc and initialises it to the null descriptor.
pascal void FFFS_AEDisposeDesc(AEDesc * desc) {
  OSStatus junk;

  FFFS_AssertQ(desc != nil);

  junk = AEDisposeDesc(desc);
  FFFS_AssertQ(junk == noErr);

  FFFS_AENullDesc(desc);
}

//*******************************************************************************
// Initialises desc to the null descriptor (typeNull, nil).
pascal void FFFS_AENullDesc(AEDesc * desc) {
  FFFS_AssertQ(desc != nil);

  desc -> descriptorType = typeNull;
  desc -> dataHandle = nil;
}

//********************************************************************************
// A simple wrapper around CreateObjSpecifier which creates
// an object specifier from a FSRef and using formName.
pascal OSStatus FFFS_AEOCreateObjSpecifierFromFSRef(const FSRefPtr pFSRefPtr, AEDesc * pObjSpecifier) {
  OSErr anErr = paramErr;

  if (nil != pFSRefPtr) {
    CFURLRef tCFURLRef = CFURLCreateFromFSRef(kCFAllocatorDefault, pFSRefPtr);

    if (nil != tCFURLRef) {
      anErr = FFFS_AEOCreateObjSpecifierFromCFURLRef(tCFURLRef, pObjSpecifier);
      CFRelease(tCFURLRef);
    } else
      anErr = coreFoundationUnknownErr;
  }
  return anErr;
}

//********************************************************************************
// A simple wrapper around CreateObjSpecifier which creates
// an object specifier from a CFURLRef and using formName.

pascal OSStatus FFFS_AEOCreateObjSpecifierFromCFURLRef(const CFURLRef pCFURLRef, AEDesc * pObjSpecifier) {
  OSErr anErr = paramErr;

  if (nil != pCFURLRef) {
    Boolean isDirectory = CFURLHasDirectoryPath(pCFURLRef);
    CFStringRef tCFStringRef = CFURLCopyFileSystemPath(pCFURLRef, kCFURLHFSPathStyle);
    AEDesc containerDesc = {
      typeNull,
      NULL
    };
    AEDesc nameDesc = {
      typeNull,
      NULL
    };
    UniCharPtr buf = nil;

    if (nil != tCFStringRef) {
      Size bufSize = (CFStringGetLength(tCFStringRef) + (isDirectory ? 1 : 0)) * sizeof(UniChar);

      buf = (UniCharPtr) NewPtr(bufSize);

      if ((anErr = MemError()) == noErr) {
        CFStringGetCharacters(tCFStringRef, CFRangeMake(0, bufSize / 2), buf);
        if (isDirectory)(buf)[(bufSize - 1) / 2] = (UniChar) 0x003A;
      }
    } else
      anErr = coreFoundationUnknownErr;

    if (anErr == noErr)
      anErr = AECreateDesc(typeUnicodeText, buf, GetPtrSize((Ptr) buf), & nameDesc);
    if (anErr == noErr)
      anErr = CreateObjSpecifier(isDirectory ? cFolder : cFile, & containerDesc, formName, & nameDesc, false, pObjSpecifier);

    FFFS_AEDisposeDesc( & nameDesc);

    if (buf)
      DisposePtr((Ptr) buf);
  }
  return anErr;
}

pascal OSStatus FFFS_AEGetHandlerError(const AppleEvent * pAEReply) {
  OSStatus anError = noErr;
  OSErr handlerErr;

  DescType actualType;
  long actualSize;

  if (pAEReply -> descriptorType != typeNull) // there's a reply, so there may be an error
  {
    OSErr getErrErr = noErr;

    getErrErr = AEGetParamPtr(pAEReply, keyErrorNumber, typeSInt64, & actualType, &
      handlerErr, sizeof(OSErr), & actualSize);

    if (getErrErr != errAEDescNotFound) // found an errorNumber parameter
    {
      anError = handlerErr; // so return it's value
    }
  }
  return anError;
}

pascal OSStatus FFFS_AESendEventReturnData(
  const AEIdleUPP pIdleProcUPP,
    const AppleEvent * pAppleEvent,
      DescType pDesiredType,
      DescType * pActualType,
      void * pDataPtr,
      Size pMaximumSize,
      Size * pActualSize) {
  OSStatus anError = noErr;

  //  No idle function is an error, since we are expected to return a value
  if (pIdleProcUPP == NULL)
    anError = paramErr;
  else {
    AppleEvent theReply = {
      typeNull,
      NULL
    };
    AESendMode sendMode = kAEWaitReply;

    anError = AESend(pAppleEvent, & theReply, sendMode, kAEHighPriority, kNoTimeOut, pIdleProcUPP, NULL);
    //  [ Don't dispose of the event, it's not ours ]
    if (noErr == anError) {
      anError = FFFS_AEGetHandlerError( & theReply);
      if (!anError && theReply.descriptorType != typeNull) {
        anError = AEGetParamPtr( & theReply, keyDirectObject, pDesiredType,
          pActualType, pDataPtr, pMaximumSize, pActualSize);
      }
      FFFS_AEDisposeDesc( & theReply);
    }
  }
  return anError;
}