/* wrapper.c --- WRAPPER

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

#include "mica_c/kvs.h"

void mica_local_read(mica_op_t *kv_ptr,
                                  uint8_t *value_to_read,
                                  uint8_t *resp_type,
		    uint16_t t_id) {
	KVS_local_read(kv_ptr, value_to_read, resp_type,t_id);
}

void mica_write(mica_op_t *kv_ptr, uint8_t *value_to_write) {
	KVS_write(kv_ptr,value_to_write);
}
/* wrapper.c ends here */
