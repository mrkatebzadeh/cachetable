/* hrd.c --- HRD

 *
 * Author: M.R.Siavash Katebzadeh <mr.katebzadeh@gmail.com>
 * Keywords: C
 * Version: 0.0.1
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include "hrd.h"

/* Allocate SHM with @shm_key, and save the shmid into @shm_id_ret */
void* hrd_malloc_socket(int shm_key, int size, int socket_id)
{
	int shmid;
	if(USE_HUGE_PAGES == 1)
		shmid = shmget(shm_key,
			size, IPC_CREAT | IPC_EXCL | 0666 | SHM_HUGETLB);
	else
		shmid = shmget(shm_key,
			size, IPC_CREAT | IPC_EXCL | 0666);

	if(shmid == -1) {
		switch(errno) {
			case EACCES:
        my_printf(red, "HRD: SHM malloc error: Insufficient permissions."
                       " (SHM key = %d)\n", shm_key);
				break;
			case EEXIST:
        my_printf(red, "HRD: SHM malloc error: Already exists."
                       " (SHM key = %d)\n", shm_key);
				break;
			case EINVAL:
        my_printf(red, "HRD: SHM malloc error: SHMMAX/SHMIN mismatch."
                       " (SHM key = %d, w_size = %d)\n", shm_key, size);
				break;
			case ENOMEM:
        my_printf(red, "HRD: SHM malloc error: Insufficient memory."
                       " (SHM key = %d, w_size = %d)\n", shm_key, size);
				break;
			case ENOENT:
        my_printf(red, "HRD: SHM malloc error: No segment exists for the given key, and IPC_CREAT was not specified."
                       " (SHM key = %d, w_size = %d)\n", shm_key, size);
				break;
			case ENOSPC:
        my_printf(red,
            "HRD: SHM malloc error: All possible shared memory IDs have been taken or the limit of shared memory is exceeded."
                " (SHM key = %d, w_size = %d)\n", shm_key, size);
				break;
			case EPERM:
        my_printf(red, "HRD: SHM malloc error: The SHM_HUGETLB flag was specified, but the caller was not privileged"
                       " (SHM key = %d, w_size = %d)\n", shm_key, size);
				break;
			case ENFILE:
        my_printf(red, "HRD: SHM malloc error: The system-wide limit on the total number of open files has been reached."
                       " (SHM key = %d, w_size = %d)\n", shm_key, size);
				break;
			default:
        my_printf(red, "HRD: SHM malloc error: A wild SHM error: %s.\n",
                   strerror(errno));
				break;
		}
		assert(false);
	}

	void *buf = shmat(shmid, NULL, 0);
	if(buf == NULL) {
		printf("HRD: SHM malloc error: shmat() failed for key %d\n", shm_key);
		exit(-1);
	}

	/* Bind the buffer to this socket */
	const unsigned long nodemask = (1 << socket_id);
	int ret = mbind(buf, size, MPOL_BIND, &nodemask, 32, 0);
	if(ret != 0) {
		printf("HRD: SHM malloc error. mbind() failed for key %d\n", shm_key);
		exit(-1);
	}

	// try to take advantage of TLB coalescing, if it is there
	if (LEVERAGE_TLB_COALESCING) {
		int page_no = CEILING(size, HUGE_PAGE_SIZE);
		int i;
		for (i = 0; i < page_no; i++)
			memset(buf + i * HUGE_PAGE_SIZE, 0, 1);
	}

	return buf;
}

/* hrd.c ends here */
