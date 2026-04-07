#include <sys/utsname.h>
#include <string.h>
#include "libafl/hooks/syscall.h"

target_ulong uname_hook(int64_t data, target_ulong ret, int sys_num, target_ulong arg0,
	target_ulong arg1, target_ulong arg2, target_ulong arg3, target_ulong arg4,
	target_ulong arg5, target_ulong arg6, target_ulong arg7)
{
	if(63 == sys_num)
	{
		printf("found uname call\n");
		struct utsname* sys_info = (struct utsname*)arg0;
		strncpy(sys_info->sysname, "hiitest", sizeof("hiitest"));
		
	}
	return 0;
}

void hook_setup()
{
	libafl_add_post_syscall_hook(uname_hook, 0);
}