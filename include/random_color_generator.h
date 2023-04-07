#pragma once

/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#if defined(__APPLE__)
#include "TargetConditionals.h"
#endif


#if defined(TARGET_OS_OSX)
typedef struct Rgb {
  double red;
  double green;
  double blue;
} Rgb;
#endif

#if defined(TARGET_OS_OSX)
/**
 * This function frees a Rust created C String
 *
 * # Safety
 *
 * This function dereferences a pointer
 */
void freeRustCString(const char *string);
#endif

#if defined(TARGET_OS_OSX)
const char *randomHex(void);
#endif

#if defined(TARGET_OS_OSX)
/**
 * This generates a random `Rgb` object
 */
struct Rgb randomRgb(void);
#endif
