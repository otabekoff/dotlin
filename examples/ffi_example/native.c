#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// Simple C function to be called from Dotlin
int add_numbers(int a, int b)
{
    return a + b;
}

// Work with strings
char *reverse_string(const char *input)
{
    if (input == NULL)
        return NULL;

    size_t len = strlen(input);
    char *result = (char *)malloc(len + 1);

    for (size_t i = 0; i < len; i++)
    {
        result[i] = input[len - 1 - i];
    }
    result[len] = '\0';

    return result;
}

// Work with arrays
void process_array(int *arr, size_t len)
{
    for (size_t i = 0; i < len; i++)
    {
        arr[i] = arr[i] * 2;
    }
}

// Return struct data
typedef struct
{
    int x;
    int y;
    double distance;
} Point;

Point create_point(int x, int y)
{
    Point p;
    p.x = x;
    p.y = y;
    p.distance = sqrt(x * x + y * y);
    return p;
}

// Callback example
typedef void (*callback_fn)(int);

void process_with_callback(int *arr, size_t len, callback_fn callback)
{
    for (size_t i = 0; i < len; i++)
    {
        callback(arr[i]);
    }
}

// Memory management helper
void free_string(char *str)
{
    if (str != NULL)
    {
        free(str);
    }
}

// Complex computation (CPU-intensive)
double compute_pi_monte_carlo(int iterations)
{
    int inside_circle = 0;

    for (int i = 0; i < iterations; i++)
    {
        double x = (double)rand() / RAND_MAX;
        double y = (double)rand() / RAND_MAX;

        if (x * x + y * y <= 1.0)
        {
            inside_circle++;
        }
    }

    return 4.0 * inside_circle / iterations;
}