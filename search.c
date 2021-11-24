#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#define N 30

void refresh(int data[])
{
    for (int i = 0; i < N; i++)
    {
        srand(time(NULL) * (i + 1));
        data[i] = rand() % 100 + 1;
    }
}
int sekuensial(int key, int data[])
{
    int i = 0, ketemu = 0;
    while (!ketemu && i < N)
    {
        if (key == data[i])
            ketemu = 1;
        else
            i++;
    }
    if (ketemu)
        return i;
    return -1;
}

int sekuensial_urut(int key, int data[])
{
    int i = 0, ketemu = 0;
    while (!ketemu && i < N)
    {
        if (key == data[i])
            ketemu = 1;
        else if (key < data[i])
            break;
        else
            i++;
    }
    if (ketemu)
        return i;
    return -1;
}

int biner(int key, int data[])
{
    int left = 0, right = N - 1, med, ketemu = 0;
    while (!ketemu && left < right)
    {
        med = (left + right) / 2;
        if (key == data[med])
            ketemu = 1;
        else if (key < data[med])
            right = med - 1;
        else
            left = med + 1;
    }
    if (ketemu)
        return med;
    return -1;
}

int partisi(int A[], int left, int right)
{
    int pivot = A[left], temp, i = left, j = right;
    while (i <= j)
    {
        while (pivot > A[i])
            i++;
        while (pivot < A[j])
            j--;
        if (i < j)
        {
            temp = A[i];
            A[i] = A[j];
            A[j] = temp;
            i++;
            j--;
        }
        else
            return j;
    }
    return j;
}
void quick(int A[], int left, int right)
{
    int q;
    if (left < right)
    {
        q = partisi(A, left, right);
        quick(A, left, q);
        quick(A, q + 1, right);
    }
}

int main(void)
{
    int data[N], i, key, pilih, indeks;
    for (i = 0; i < N; i++)
    {
        srand(time(NULL) * (i + 1));
        data[i] = rand() % 100 + 1;
    }
    while (1)
    {
        for (i = 0; i < N; i++)
        {
            printf("%d ", data[i]);
        }
        printf("\n1. sekuensial\n");
        printf("2. biner\n");
        printf("3. sekuensial urut\n");
        printf("98.refresh\n");
        printf("99.keluar\n");
        printf("masukkan pilihan: ");
        scanf("%d", &pilih);
        if (pilih < 97)
        {

            printf("\nmasukkan key: ");
            fflush(stdin);
            scanf("%d", &key);
        }
        switch (pilih)
        {
        case 1:
            indeks = sekuensial(key, data);
            break;
        case 2:
            quick(data, 0, N - 1);
            indeks = biner(key, data);
            break;
        case 3:
            quick(data, 0, N - 1);
            indeks = sekuensial_urut(key, data);
            break;
        case 98:
        {
            refresh(data);
            break;
        }
        case 99:
            exit(1);
        default:
            break;
        }
        printf("\n");
        if (indeks >= 0)
        {
            printf("data %d ditemukan pada indeks ke %d\n", key, indeks);
        }
        else
        {
            printf("data %d tidak ditemukan\n", key);
        }
        printf("\n");
    }
    return 0;
}
