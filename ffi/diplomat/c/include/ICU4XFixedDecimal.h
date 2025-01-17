#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
#include "ICU4XCreateFixedDecimalResult.h"

ICU4XFixedDecimal* ICU4XFixedDecimal_create(int32_t v);

ICU4XCreateFixedDecimalResult ICU4XFixedDecimal_create_fromstr(const char* v_data, size_t v_len);

bool ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

void ICU4XFixedDecimal_negate(ICU4XFixedDecimal* self);

void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWriteable* to);
void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);

#ifdef __cplusplus
}
#endif
#endif
