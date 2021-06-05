#ifndef _VEC_H_
#define _VEC_H_

#include <string.h>
#include <stdint.h>
#include <stdlib.h>
#include <assert.h>

typedef struct {
    int32_t * content;
    size_t capacity;
    size_t size;
} Vec;
static inline Vec *
Vec_with_capacity(size_t capacity)
{
    Vec * to_ret = (Vec *)malloc(sizeof(Vec));
    if (to_ret == NULL)
    {
        return NULL;
    }
    to_ret->content = (int32_t *)malloc(32 * capacity);
    if (to_ret->content == NULL)
    {
        free(to_ret);
        return NULL;
    }
    to_ret->capacity = capacity;
    to_ret->size = 0;
    return to_ret;
}
static inline Vec *
Vec_new(void)
{
    return Vec_with_capacity(1);
}
static inline void
Vec_resize(Vec * self, size_t n)
{
    int32_t * saved = self->content;
    self->capacity = n;
    self->content = (int32_t *)malloc(32 * self->capacity);
    memcpy(self->content, saved, self->size);
}
static inline void
Vec_push(Vec * self, int32_t n)
{
    if (self->size + 1 >= self->capacity)
        Vec_resize(self, self->capacity * 2);
    self->content[self->size++] = n;
}
static inline Vec *
Vec_from(int32_t * vals, size_t n)
{
    Vec * to_ret = Vec_with_capacity(n);
    for (size_t idx = 0; idx < n; idx++)
    {
        Vec_push(to_ret, vals[idx]);
    }
    return to_ret;
}
static inline void
Vec_pop(Vec * self)
{
    self->size--;
}
static inline void
Vec_clear(Vec * self)
{
    free(self->content);
    self->size = 0;
    self->capacity = 1;
}
static inline void
Vec_remove(Vec * self, size_t idx)
{
    memcpy(&self->content[idx], &self->content[idx + 1], self->size - (self->size - idx));
}
static inline void
Vec_insert(Vec * self, int32_t val, size_t idx)
{
    if (self->size + 1 >= self->capacity)
        Vec_resize(self, self->capacity * 2);
    memcpy(&self->content[idx + 1], &self->content[idx], self->size - (self->size - idx));
    self->content[idx] = val;
}
static inline void
Vec_shrink_to_fit(Vec * self)
{
    self->capacity = self->size;
    free(&self->content[self->capacity - 1]);
}
static inline int32_t
Vec_empty(Vec * self)
{
    return self->size == 0;
}
#endif
