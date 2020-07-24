#include <stdio.h>
#include <stdlib.h>
#include <gsl/gsl_histogram.h>
#include <math.h>

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

// Global variable
long l_seed;

double getRandom();  // get random number
double getGaussian();  // get random gaussian

int main (){
    int i;
    FILE *uniform = fopen("images/histogram_uniform.dat", "w");
    FILE *gaussian = fopen("images/histogram_gaussian.dat", "w");

    l_seed = (long)1231412314;

    for(i = 0; i < 1000000; i++){
        fprintf(uniform, "%.10e\n", getRandom());
    }
    fclose(uniform);

    for(i = 0; i < 1000000; i++){
        fprintf(gaussian, "%.10e\n", getGaussian());
    }
    fclose(gaussian);
}


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
