#include "vec.h"
#include <stdio.h>

int
main(void)
{
    Vec * v = Vec_new();
    Vec_push(v, 4);
    assert(v->content[0] == 4);
    Vec_pop(v);
    assert(v->size == 0);
    Vec_push(v, 0);
    Vec_push(v, 1);
    Vec_push(v, 2);
    Vec_remove(v, 1);
    assert(v->content[1] == 2);
    Vec_insert(v, 42, 0);
    assert(v->content[0] == 42);
    Vec_pop(v);
    Vec_pop(v);
    assert(v->capacity == 8);
    Vec_shrink_to_fit(v);
    assert(v->capacity == 1);
    assert(Vec_empty(v) == 0);
    int32_t aa[] = { 1, 2, 3 };
    Vec * a = Vec_from(aa, 3);
    assert(a->content[0] == 1);
    return EXIT_SUCCESS;
}
