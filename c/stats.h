#ifndef STATS_H
#define STATS_H

#include <string.h>
#include <stdbool.h>

static inline void
sort(double array[], size_t size)
{
		for (size_t i = 1; i < size; i++)
		{
				bool sorted = true;
				for (size_t j = 0; j < i; j++)
				{
						if (array[j + 1] < array[j])
						{
								double tmp = array[j];
								array[j] = array[j + 1];
								array[j + 1] = tmp;
								sorted = false;
						}
				}
				if (sorted)
						break;
		}
}

#endif
