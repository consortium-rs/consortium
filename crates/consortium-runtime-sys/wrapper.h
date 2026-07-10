/* Copyright 2026 Ethan Wu
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

/*
 * Bindgen entry point for `rsc_table.h`. The upstream header is written against
 * the Linux kernel environment, so supply the handful of kernel spellings it
 * relies on (fixed-width type aliases, the `__packed` attribute) so libclang can
 * parse it standalone. The trailing `rsc_table_for_each_entry()` inline helper
 * also references the kernel device/error plumbing; provide minimal stand-ins so
 * the file parses. Bindgen does not emit a binding for the `static inline`
 * function, so these stubs never reach the generated Rust.
 */

#ifndef CONSORTIUM_RUNTIME_SYS_WRAPPER_H
#define CONSORTIUM_RUNTIME_SYS_WRAPPER_H

#include <stddef.h>
#include <stdint.h>

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

#ifndef __packed
#define __packed __attribute__((packed))
#endif

struct device;
#ifndef dev_err
#define dev_err(dev, ...) ((void)(dev))
#endif
#ifndef EINVAL
#define EINVAL 22
#endif

#include "rsc_table.h"

#endif /* CONSORTIUM_RUNTIME_SYS_WRAPPER_H */
