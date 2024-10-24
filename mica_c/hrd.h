/* hrd.h --- HRD

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

#ifndef HRD_H
#define HRD_H
#include <sys/ipc.h>
#include <sys/shm.h>
#include <numaif.h>

#include "defines.h"
#include "utils.h"

void* hrd_malloc_socket(int shm_key, int size, int socket_id);

#endif /* HRD_H */
/* hrd.h ends here */
