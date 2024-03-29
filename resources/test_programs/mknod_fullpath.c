#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <err.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/syscall.h>   /* For SYS_xxx definitions */

// Use absolute path to file, instead of relative path.

int withError(int returnCode, char* call);
#define file "mknod.txt"

int main(){
  // Make the file once so we can realpath it, and then copy it over.
  withError(mknod(file, S_IFREG, 0), "cannot mknod");
  char* fullpath = realpath(file, NULL);
  unlink(fullpath);
  withError(mknod(fullpath, S_IFREG, 0), "cannot mknod");

  struct stat myStat;
  withError(lstat(file, &myStat), "stat");
  time_t mtime = myStat.st_mtime;
  withError(unlink(file), "Unlink "file);

  // Second file created.
  printf("mtime %ld\n", mtime);

  return 0;
}

int withError(int returnCode, char* call){
  if(returnCode == -1){
    printf("Unable to %s: %s", call, strerror(errno));
    exit(1);
  }

  return returnCode;
}
