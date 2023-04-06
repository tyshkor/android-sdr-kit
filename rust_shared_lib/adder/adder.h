#ifndef __cplusplus
#include <stdbool.h>
#endif
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

int32_t add_numbers(int32_t a, int32_t b);
int32_t compress_array(int32_t number_lines, float* data);

#ifdef __cplusplus
}  // extern "C"
#endif