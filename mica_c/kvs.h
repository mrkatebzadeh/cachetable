/* kvs.h --- KVS

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

#ifndef KVS_H
#define KVS_H

#ifndef _GNU_SOURCE
# define _GNU_SOURCE
#endif

#include <stdint.h>
#include <stdatomic.h>
#include <assert.h>
#include <stdbool.h>
#include <string.h>
#include <stdarg.h>
#include <stdio.h>
#include "sizes.h"
#include "defines.h"
#include "seqlock.h"
#include "hrd.h"
#include "utils.h"

//KVS Response
typedef enum {
  EMPTY = 120,
  KVS_GET_TS_SUCCESS = 21,
  KVS_GET_SUCCESS = 121,
  KVS_PUT_SUCCESS = 122,
  KVS_LOCAL_GET_SUCCESS = 123,
  KVS_GET_OP_BUFFER = 124,
  KVS_PUT_OP_BUFFER = 125,
  KVS_MISS = 130
} resp_type_t;


typedef struct {
  uint8_t value[MICA_VALUE_SIZE];
  mica_key_t key;
  seqlock_t seqlock;
  uint64_t version;
  uint8_t m_id;
  uint8_t state;
  uint8_t unused[2];
  uint32_t key_id; // strictly for debug
  uint8_t padding[MICA_OP_PADDING_SIZE];
} mica_op_t;

struct mica_slot {
	uint32_t in_use	:1;
	uint32_t tag	:(64 - MICA_LOG_BITS - 1);
	uint64_t offset	:MICA_LOG_BITS;
};

struct mica_bkt {
	struct mica_slot slots[8];
};

typedef struct  {
	struct mica_bkt *ht_index;
	uint8_t *ht_log;

	/* Metadata */
	int instance_id;	/* ID of this MICA instance. Used for shm keys */
	int node_id;

	int num_bkts;	/* Number of buckets requested by user */
	int bkt_mask;	/* Mask down from a mica_key's @bkt to a bucket */

	uint64_t log_cap;	/* Capacity of circular log in bytes */
	uint64_t log_mask;	/* Mask down from a slot's @offset to a log offset */

	/* State */
	uint64_t log_head;

	/* Stats */
	long long num_get_op;	/* Number of GET requests executed */
	long long num_put_op;	/* Number of PUT requests executed */
	long long num_get_fail;	/* Number of GET requests failed */
	long long num_put_fail;	/* Number of GET requests failed */
	long long num_insert_op;	/* Number of PUT requests executed */
	long long num_index_evictions; /* Number of entries evicted from index */
} mica_kv_t;

extern mica_kv_t *KVS;



void custom_mica_init(int kvs_id);
void custom_mica_populate_fixed_len(mica_kv_t *, int n, int val_len);


/* ---------------------------------------------------------------------------
//------------------------------ KVS UTILITY GENERIC -----------------------------
//---------------------------------------------------------------------------*/

static inline void od_KVS_check_key(mica_op_t *kv_ptr, mica_key_t opkey, uint32_t op_i)
{
	if (ENABLE_ASSERTIONS && kv_ptr == NULL) assert(false);
	bool key_found = memcmp(&kv_ptr->key, &opkey, KEY_SIZE) == 0;
	if (unlikely(ENABLE_ASSERTIONS && !key_found)) {
		my_printf(red, "Kvs miss %u\n", op_i);
		cust_print_key("Op", &opkey);
		cust_print_key("KV_ptr", &kv_ptr->key);
		assert(false);
	}
}


static inline void KVS_local_read(mica_op_t *kv_ptr,
                                  uint8_t *value_to_read,
                                  uint8_t *resp_type,
                                  uint16_t t_id)
{
  if (ENABLE_ASSERTIONS) {
    assert(value_to_read != NULL);
    assert(kv_ptr != NULL);
  }
  uint32_t debug_cntr = 0;
  uint64_t tmp_lock = read_seqlock_lock_free(&kv_ptr->seqlock);
  do {
    debug_stalling_on_lock(&debug_cntr, "local read", t_id);
    memcpy(value_to_read, kv_ptr->value, (size_t) VALUE_SIZE);
  } while (!(check_seqlock_lock_free(&kv_ptr->seqlock, &tmp_lock)));
	if (resp_type != NULL)
  	*resp_type = KVS_LOCAL_GET_SUCCESS;
}

static inline void KVS_write(mica_op_t *kv_ptr, uint8_t *value_to_write)
{
	lock_seqlock(&kv_ptr->seqlock);
		memcpy(kv_ptr->value, value_to_write, (size_t) VALUE_SIZE);
	unlock_seqlock(&kv_ptr->seqlock);
}



#endif /* KVS_H */
/* kvs.h ends here */
