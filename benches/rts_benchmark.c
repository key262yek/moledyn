/*

Diffusion limited simulation.
Infinite well potential for clustering

no_speed_upV1.0
Input
    System info(System size, Target size, Dimension, Number of Searchers, merging threshold distance, slowing down exponent),
    Simulation Info(Time scale, repetition, set number, random seed, directory to save files)

Process
    - Initiate
    - Particles Move
    - Merging
    - Target Search
    - Terminate and save data.
(No additional speed up technique)

Result
    - Random seed of simulation
    - Searching Time.

Check
    - Initial distribution - Uniform distribution confirmed

mu0_exponentially_increaseV1.0.c
New process
    - mu0 changes exponentially
    - d_min_mu0 to d_max_mu0 (ex. 1e-10 to 1e-3)
    - each incerase ratio is defined by d_inc_ratio (ex. 10)
    - each mu0 does a role with 3 * d_inc_ratio steps. (ex. 40)

    - Skip Merge function for rth=0 case.

Check
    - Searching time distribution with original one
        - Confirmed by no_speedV1.0.c results with mu0=1e-6 (20200120 first trial)

cluster_distributionV1.0
New process
    - Remove target and search check
    - Plot cluster distribution and effective number of particle.

*/


// Headers
#include <stdio.h>  // Standard input output
#include <stdlib.h>  // Standard library
#include <float.h>  // For DBL_MAX
#include <math.h>  // Mathematical functions like exponential, logarithm
#include <string.h>  // To use strcat()
#include <time.h>  // For clock() function
#include <dirent.h>  // directory entry
#include <sys/types.h>
#include <sys/stat.h>  // Check existance of directory
#include <sys/time.h>  // For clock() function
#include <unistd.h>

// Constants for random number generator
#define IM1 2147483563
#define IM2 2147483399
#define AM (1.0/IM1)
#define IMM1 (IM1-1)
#define IA1 40014
#define IA2 40692
#define IQ1 53668
#define IQ2 52774
#define IR1 12211
#define IR2 3791
#define NTAB 32
#define NDIV (1+IMM1/NTAB)
#define EPS 1.2e-7
#define RNMX (1.0-EPS)

// Constants
#define PI 3.14159265358979323846264338327950288419716939937510


// Global variables

// System configuration
int i_dim;
double d_sys_size, d_target_size, d_volume, d_max_step;
double d_run_time, d_lambda, d_lambda_ratio;
clock_t start, end;

/*
i_dim : dimension of system;
i_bc : boundary condition of system (0 : periodic, 1 : reflection)
i_sys_shape : shape of system (0 : rectangular, 1 : circular, spherical)

d_sys_size : size of system
d_target_size : size of target
d_volume : volume of system
d_max_step : limit of random step. we should truncate such big random number.

d_lambda : the smallest eigenvalue of diffusion equation
d_lambda_ratio : finite system correction rate

start, end : time info of start, end
d_run_time : run time for single simulation
*/

// Simulation
int i_repeat, i_set, i_num_searcher, i_flag, i_step;
long l_org_seed, l_seed;
double d_mu_ratio, d_min_distance, d_time, d_mu0, d_org_mu0, d_merge_thr, d_density, d_alpha, d_exp_time;
double d_min_mu0, d_max_mu0, d_inc_ratio;
double d_diffusion_ratio, *da_diffusion;
double* da_single_move;
char s_file_dir[2000];
FILE* f_output;

int *ia_searching_cluster_dtb;
double d_searching_time;

/*
i_repeat : number of repetition of simulation
i_set : index of simulation set
i_num_searcher : initial number of searcher
i_flag : did searchers find a target? 0 : no, 1 : yes
l_org_seed : original seed. we convert it to various l_seed
l_seed : seed for pseudo random number generating

d_mu_ratio : ratio between d_min_distance and d_mu0.
            -> change time scale with respect to the distances.
d_min_distance : minimal distance between searchers or target.
d_time : searching time
d_exp_time : expected searching time computed by survival probability
d_mu0 : time scale, diffusion coefficient
d_merge_thr : threshold radius to merge
d_density : initial density of system
d_alpha : diffusion decay exponent. diffusion coefficient decreases with factor pow(weight, -alpha)
d_diffusion_ratio : diffusion coefficient between particle.

da_single_move : single displacement vector.

s_file_dir : string of file directory
f_output : output file pointer
*/


// Clusters
struct Cluster{
    int weight, dead;
    int cell;
    double *position;
};

int i_min_weight, i_merge_flag, i_num_cluster;
int **ia_linked_list, i_head, i_tail;
struct Cluster cluster, *ca_clusters;

/*
weight : weight of clusters.
dead : flag for dead or alive. dead : 1, alive : 0
cell : index of contained cell.

position : position of cluster

ia_linked_list : linked list of clusters;
i_head : index of head cluster of linked list
i_tail : index of tail cluster of linked list

ca_clusters : array of cluster.
*/


// Plot
int i_num_plot_points, i_current_slot;
double d_next_plot, d_plot_inc, d_plot_starttime, d_plot_endtime;
double d_org_plot_dur;
double d_eff_number, d_nearest_varied_time, d_surv_prob, d_exp_time;
int *ia_current_cluster_dtb, *ia_element, *ia_survival_system;
double **da_cluster_dtb, *da_survival_prob;

/*
i_num_plot_points : number of times to plot

d_next_plot : time to next plot
d_plot_inc : time ratio between plot
d_plot_endtime : end time to plot

d_eff_number : effective number of searcher
d_nearest_varied_time : the time at which the effective number of searcher varied.
d_surv_prob : survival probability.

ia_current_cluster_dtb[k] : number of k-cluster at current time. k = 1, ..., i_num_searcher
da_cluster_dtb[t][k] : averaged cluster distribution. number of k-cluster at time t.
da_survival_prob[t] : averaged survival probability at time t.
*/


// Functions

void Save(int);  // save searching data
void Output();  // export searching time, running time, searching cluster, cluster distribution

double getRandom();  // get random number
double getGaussian();  // get random gaussian
void getRandomArray();  // get array of random Gaussian numbers.

void IndivMove();  // individual move of particles without cell method
void checkArrive_n_BC();  // check arrive and Boundary condition
void Move();  // Movement of particles.

void Run();  // Simulation step
void Initiate();  // Initiation of variables.
void setNewSimulation();  // reset simulations.

double get_time()
{
    struct timeval t;
    struct timezone tzp;
    gettimeofday(&t, &tzp);
    return t.tv_sec + t.tv_usec*1e-6;
}

// Program start.

int main(){
    int idx, i, j;
    double start, end;

    // Read argument
    d_sys_size = 10.0;
    d_target_size = 1.0;
    i_dim = 2;

    i_num_searcher = 1;
    d_mu0 = 1e-3;
    i_repeat = 100;
    i_set = 10;
    l_org_seed = 1231423;

    start = get_time();
    for(j = 0; j <i_set; j++){
        Initiate();
        for(i = 0; i < i_repeat; i++){
            l_seed += i;  // change random numbers
            setNewSimulation();
            Run();
        }
    }
    end = get_time();
    printf("time for 100 ensemble : %.5es/iter", (end - start) / i_set);
}


//==============================================================================//
//======================= Main Structure of Simulation==========================//
//==============================================================================//


void Initiate(){
    // Initiate variables and arrays
    int i, j;
    struct stat st = {0};

    // Modify variables
    i_num_cluster = i_num_searcher;
    d_max_step = 10 * d_sys_size;

    // Make seed depends on the argument. integers are all primes.
    l_seed = (long)(l_org_seed + i_num_searcher * 5413 +\
                   i_set * 733 + i_repeat * 13);

    cluster.weight = 1;
    cluster.dead = 0;
    cluster.position = (double *) calloc(i_dim, sizeof(double));

    da_single_move = (double *) calloc(i_dim, sizeof(double));
}


void setNewSimulation(){
    // Reset simulation info.
    int i, j;
    double r, x, ts2, ss2;

    // initiate time and flag
    d_time = 0; d_mu0 = d_mu0;
    i_step = 0; i_flag = 0;

    // initiate searchers' info
    ss2 = pow(d_sys_size, 2.0);
    ts2 = pow(d_target_size, 2.0);
    // reset position
    r = 2 * ss2;
    while(r > ss2 || r < ts2){
        // if searcher is already on the target
        // or it is outside of the boundary (in spherical system only)
        r = 0;
        for(j = 0; j < i_dim; j++){
            x = d_sys_size * (2 * getRandom() - 1);
            cluster.position[j] = x;
            r += x * x;
        }
        r = sqrt(r);
    }

}


void Run(){
    int t;

    while(i_flag == 0){
        d_time += d_mu0; // i_step += 1;
        Move();
    }
}

void Move(){
    int i;
    IndivMove();
    checkArrive_n_BC();
}


void IndivMove(){
    // Individual moving without cell method
    int j, weight;
    double step, x;

    // Diffusion
    step = sqrt(2 * d_mu0);  // step size
    getRandomArray();  // get Random Gaussian array to move into da_single_move[]
    for(j = 0; j < i_dim; j++){
        da_single_move[j] *= step;
    }

    for(j = 0; j < i_dim; j++){
        x = da_single_move[j];
        cluster.position[j] += x;
    }
}


//==============================================================================//
//============================== Check Functions ===============================//
//==============================================================================//

void checkArrive_n_BC(){
    int j;
    double s, r;

    // Small step approximation - Eventually Gaussian makes big step.
    r = 0;
    for(j = 0; j < i_dim; j++){
        s = cluster.position[j];
        r += s * s;
    }
    r = sqrt(r);
    if(r > d_sys_size){
        s = (2 * d_sys_size - r) / r;
        for(j = 0; j < i_dim; j++){
            cluster.position[j] *= s;
        }
    }
    else if(r < d_target_size){
        i_flag = 1;
        return;
    }
}


//==============================================================================//
// compute random variables or change simulation variables (such as jump)=======//
//==============================================================================//


double getRandom(){
    // return uniform random number in [0, 1]
    int j;
    long k;
    static long idum2=123456789;
    static long iy=0;
    static long iv[NTAB];
    float temp;

    if (l_seed <= 0) {
        if (- l_seed < 1) l_seed=1;
        else l_seed = - l_seed ;
        idum2 = l_seed;
        for (j = NTAB + 7; j >= 0; j--) {
            k = l_seed / IQ1;
            l_seed = IA1 * (l_seed - k * IQ1) - k * IR1;
            if (l_seed < 0) l_seed += IM1;
            if (j < NTAB) iv[j] = l_seed;
        }
        iy=iv[0];
    }
    k = l_seed / IQ1;
    l_seed = IA1 * (l_seed - k * IQ1) - k * IR1;
    if (l_seed < 0) l_seed += IM1;
    k= idum2 / IQ2;
    idum2 = IA2 * (idum2 - k * IQ2) - k * IR2;
    if (idum2 < 0) idum2 += IM2;
    j = iy / NDIV;
    iy = iv[j] - idum2;
    iv[j] = l_seed;
    if (iy < 1) iy += IMM1;
    if ((temp = AM * iy) > RNMX) return RNMX;
    else return temp;
}


double getGaussian(){
    // return random number of Normal Gaussian.
    static int iset=0;
    static double gset;
    double fac,rsq,v1,v2;

    if (l_seed < 0) iset=0;
    if (iset == 0) {
        do {
            v1 = 2.0 * getRandom()-1.0;
            v2 = 2.0 * getRandom()-1.0;
            rsq = v1 * v1 + v2 * v2;
        } while (rsq >= 1.0 || rsq == 0.0);
        fac = sqrt(- 2.0 * log(rsq) / rsq);
        gset = v1 * fac;
        iset = 1;
        return v2 * fac;
    } else {
        iset = 0;
        return gset;
    }
}

void getRandomArray(){
    int i;
    double x;
    for(i = 0; i < i_dim; i++){
        x = 2 * d_max_step;
        while(x > d_max_step)
            x = getGaussian();
        da_single_move[i] = x;  // change da_single_move
    }
}




