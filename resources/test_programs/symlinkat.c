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

int withError(int returnCode, char* call);
#define symlink_target "./symlinkat_target.txt"
#define symlink_name "./symlinkat.txt"

// test symlink modified time.

int main(){
  system("touch "symlink_target );
  char* fulltarget = realpath(symlink_target, NULL);
  int dirfd = withError(open(".", O_DIRECTORY | O_RDONLY), "cannot open dirfd");
  if (fulltarget == NULL) {
    printf("Unable to get full path.");
    return 1;
  }

  withError(symlinkat(fulltarget, dirfd, symlink_name), "create symlink");

  struct stat myStat;
  withError(lstat(symlink_name, &myStat), "stat");
  time_t mtime = myStat.st_mtime;
  ino_t inode = myStat.st_ino;

  withError(unlink(symlink_target), "Unlink "symlink_target);
  withError(unlink(symlink_name), "Unlink symlink");
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
