/* seqlock.h --- SEQLOCK

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

#ifndef SEQLOCK_H
#define SEQLOCK_H

#include "defines.h"

#define DISABLE_LOCKING 0
#define ENABLE_LOCK_FREE_READING 1
#define WORKERS_PER_MACHINE 8
#define DEBUG_SEQLOCKS 0

typedef atomic_uint_fast64_t seqlock_t;

static inline void lock_seqlock(seqlock_t *seqlock);
static inline void unlock_seqlock(seqlock_t *seqlock);

static inline bool is_odd(uint64_t var) {
  return ((var % 2) == 1);
}

static inline bool is_even(uint64_t var) {
  return ((var % 2) == 0);
}

static inline void debug_stalling_on_lock(uint32_t *debug_cntr, const char *message, uint16_t t_id)
{
  if (ENABLE_ASSERTIONS) {
    (*debug_cntr)++;
    if ((*debug_cntr) == M_4) {
      printf("Worker %u stuck on %s \n", t_id, message);
      (*debug_cntr) = 0;
    }
  }
}

// return true if the check was successful (loop while it returns false!)
static inline bool check_seqlock_lock_free(seqlock_t *seqlock,
                                           uint64_t *read_lock)
{
  if (DISABLE_LOCKING) return true;
  if (!ENABLE_LOCK_FREE_READING) {
    unlock_seqlock(seqlock);
    return true;
  }
  COMPILER_BARRIER();
  uint64_t tmp_lock = (uint64_t) atomic_load_explicit (seqlock, memory_order_acquire);
  if (*read_lock == tmp_lock) return true;
  else {
    while (is_odd(tmp_lock)) {
      tmp_lock = (uint64_t) atomic_load_explicit (seqlock, memory_order_acquire);
    }
    *read_lock = tmp_lock;
    return false;
  }
}

static inline void lock_seqlock(seqlock_t *seqlock)
{
  if (WORKERS_PER_MACHINE == 1) return;
  if (DISABLE_LOCKING) return;
  uint64_t tmp_lock, new_lock;
  tmp_lock = (uint64_t) atomic_load_explicit(seqlock, memory_order_acquire);
  do {
    // First spin in your L1, reading until the lock is even
    while (is_odd(tmp_lock)) {
      tmp_lock = (uint64_t) atomic_load_explicit(seqlock, memory_order_acquire);
    }

    new_lock = tmp_lock + 1;
    if (DEBUG_SEQLOCKS) assert(is_odd(new_lock));
  } while(!(atomic_compare_exchange_strong_explicit(seqlock, &tmp_lock,
                                                    new_lock,
                                                    memory_order_acquire,
                                                    memory_order_acquire)));
  if (DEBUG_SEQLOCKS) assert(is_odd ((uint64_t) atomic_load_explicit (seqlock, memory_order_acquire)));


}

static inline void unlock_seqlock(seqlock_t *seqlock)
{
  if (WORKERS_PER_MACHINE == 1) return;
  if (DISABLE_LOCKING) return;
  uint64_t tmp = *seqlock;
  if (DEBUG_SEQLOCKS) {
    assert(is_odd(tmp));
  }
  atomic_store_explicit(seqlock, tmp + 1, memory_order_release);
  //asm volatile ("clflush (%0)" :: "r"(seqlock));
}
// LOCK-free read
static inline uint64_t read_seqlock_lock_free(seqlock_t *seqlock)
{
  if (DISABLE_LOCKING) return 0;
  if (!ENABLE_LOCK_FREE_READING) {
    lock_seqlock(seqlock);
    return 0;
  }
  uint64_t tmp_lock;
  do {
    tmp_lock = (uint64_t) atomic_load_explicit (seqlock, memory_order_acquire);
  } while (is_odd(tmp_lock));

  return tmp_lock;
}
#endif /* SEQLOCK_H */
/* seqlock.h ends here */
