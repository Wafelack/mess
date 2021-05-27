#include <stdio.h>
#include "stats.h"

int
main(void)
{
		double array[] = {1.4, 0.5, 0.6, 1.5, 0.7, 0.3};
		sort(array, 6);
		for (size_t i = 0; i < 6; i++)
		{
				printf("%f\n", array[i]);
		}
		return 0;
}
