/* defines.h --- DEFINES

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

#ifndef DEFINES_H
#define DEFINES_H

#include <stdlib.h>
#include "sizes.h"

#define COMPILER_BARRIER() asm volatile ("" ::: "memory")
#define FIND_PADDING_CUST_ALIGN(size, align) ((align - (size % align)) % align)
#define FIND_PADDING(size) ((64 - (size % 64)) % 64)
#define unlikely(x)       __builtin_expect((x), 0)
#define CEILING(x,y) (((x) + (y) - 1) / (y))

#define KVS_NUM_BKTS (8 * 1024 * 1024)
#define KVS_LOG_CAP  (1024 * 1024 * 1024)
#define KVS_SOCKET 0// (WORKERS_PER_MACHINE < 30 ? 0 : 1 )// socket where the cache is bind

#define MICA_LOG_BITS 40
#define MICA_INDEX_SHM_KEY 1185
#define MICA_LOG_SHM_KEY 2185

#define EXTRA_CACHE_LINES 0
#define USE_BIG_OBJECTS 0
#define BASE_VALUE_SIZE 32
#define VALUE_SIZE_ (USE_BIG_OBJECTS ? ((EXTRA_CACHE_LINES * 64) + BASE_VALUE_SIZE) : BASE_VALUE_SIZE) //(169 + 64)// 46 + 64 + 64//32 //(46 + 64)
#define VALUE_SIZE (VALUE_SIZE_ + (FIND_PADDING_CUST_ALIGN(VALUE_SIZE_, 8)))
#define MICA_VALUE_SIZE (VALUE_SIZE + (FIND_PADDING_CUST_ALIGN(VALUE_SIZE, 32)))

#define MICA_OP_SIZE_ (32 + ((MICA_VALUE_SIZE)))
#define MICA_OP_PADDING_SIZE  (FIND_PADDING(MICA_OP_SIZE_))
#define MICA_OP_SIZE  (MICA_OP_SIZE_ + MICA_OP_PADDING_SIZE)
#define KVS_NUM_KEYS (1 * MILLION)

#define ENABLE_ASSERTIONS 1
#define USE_HUGE_PAGES 0
#define LEVERAGE_TLB_COALESCING 1
#define HUGE_PAGE_SIZE 2097152
#define KEY_SIZE 8


typedef struct key {
  unsigned int bkt			:32;
  unsigned int server			:16;
  unsigned int tag			:16;
} mica_key_t;

#endif /* DEFINES_H */
/* defines.h ends here */
