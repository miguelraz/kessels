#include <stdio.h>
#include <stdlib.h>
#include <omp.h>
#include <stdbool.h>

// 2. Realiza la suma iterativa de los primeros 1000 naturales
int trabajo(int idThread)
{
    //    printf("Hilo,%i,START,JOB\n", idThread);
    int sum = 0;
    for (int i = 1; i < 1001; i++)
    {
        sum += i;
    }
    //    printf("Hilo,%i,END,JOB\n", idThread);
    //    printf("RESULT,%i\n", sum);
    return sum;
}

// Copia descarada de la practica0 para empezar el programa
int main(int argc, char **argv)
{

    int MAX_HILOS = 1024; //limite superior (arbitrario) del numero de hilos
    int nThreads, idThread;

    // 0. Recibir del usuario el numero de n hilos a utilizar
    if (argc < 2)
    {
        printf("Please specify nThreads\n");
        exit(1);
    }
    sscanf(argv[1], "%i", &nThreads);
    if (nThreads < 1 || nThreads > MAX_HILOS)
    {
        printf("nThreads not an int (%i)\n", nThreads);
        exit(1);
    }

    omp_set_num_threads(nThreads);
    long int sum;
    sum = 0;

    // Variables globales
    omp_set_num_threads(nThreads);
    int n = nThreads;
    int v = nThreads - 1;
    int edge[v];
    int competing[v][2];
    int turn[v][2];

    // Inicializar variables
    for (int i = 0; i < v; i++)
    {
        edge[i] = competing[i][0] = competing[i][1] = turn[i][0] = turn[i][1] = 0;
    }

#pragma omp parallel
    {

        int idThread = omp_get_thread_num(); // Who I am
        int node = idThread + n;             // where I am
        int local = 0;
        int id = 0;
        // Begin DOWN pass
        while (node > 1)
        {
            id = node % 2;
            node = node / 2; // Set which node I will compete for

            // Announce I am competing
            competing[node][id] = 1; // Aviso que compito (y detengo que mis hijos entren al mismo nodo)

            // Compete for the node
            local = (turn[node][1 - id] + id) % 2; // If different, 0. If same, set to 1.

            // Last one in cedes their turn.
            turn[node][id] = local;

            // Set my id and the other's id.

            // Wait to enter critical section
            // Cond 1:  The other isn't competing. Cond 2: it's your turn.

            while (!((competing[node][1 - id] == 0) || (local != (turn[node][1 - id] + id) % 2)))
            {
                ;
            };

            // Win your round
            edge[node] = id;
        }
        sum += trabajo(idThread);
        node = 1;

        // UP Pass:propagate flags so that other nodes may compete
        // Cleanup and reset flags up the tree
        while (node < n)
        {
            competing[node][edge[node]] = 0;
            node = 2 * node + edge[node];
        }
    }
}
