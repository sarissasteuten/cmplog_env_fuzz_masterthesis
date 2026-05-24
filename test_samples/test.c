#include <sys/utsname.h>
#include <sys/sysinfo.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <unistd.h>
#include <stdio.h>
#include <fcntl.h>
#include <dirent.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

int main()
{
	bool all = false; 
	int count = 0;
	struct utsname system_info;
	uname(&system_info);
	
	printf("sysname = %s\n", system_info.sysname);

	if (strcmp(system_info.sysname, "hiitest") == 0)
	{
		printf("1 YESS CHANGED uname\n");
		count++;
	}
	else
	{
		// printf("NOOOO WRONG uname\n");
	}
	
	struct sysinfo sys_info;
	sysinfo(&sys_info);
	if (sys_info.uptime == 172834)
	{
		count++;
		// printf("2 YESS CHANGED sysinfo\n");
	}
	else
	{
		// printf("NOOOO WRONG sysinfo %ld\n", sys_info.uptime);
	}
	
	if (getpid() == 5201)
	{
		count++;
		// printf("3 YESS CHANGED pid\n");
	}
	else
	{
		// printf("NOOOO WRONG pid\n");
	}
	
	if (getppid() == 3018)
	{
		count++;
		// printf("4 YESS CHANGED ppid\n");
	}
	else
	{
		// printf("NOOOO WRONG ppid\n");
	}
	
	struct stat status; 
	stat("/etc/hostname", &status); 
	if (status.st_dev == 2049)
	{
		count++;
		// printf("5 YESS CHANGED status\n");
	}
	else
	{
		// printf("NOOOO WRONG status\n");		
	}
	
	if (access("/proc/vmware", F_OK) == 1)
	{
		// printf("6 YESS CHANGED access, %d\n", access("/proc/vmware", F_OK) );
		count++;
	}
	else
	{
		// printf("NOOOO WRONG access\n");
	}
	
	struct timeval time_v; 
	gettimeofday(&time_v, NULL);
	if (time_v.tv_sec == 1700006734)
	{
		count++;
		// printf("7 YESS CHANGED time\n");
	}
	else
	{
		// printf("NOOOO WRONG time\n");
	}
	
	if (count == 7) printf("all where correct\n");
	
	return 0;
}
