#include <iostream>
#include <vector>

#include "chromosome.h"

template <typename T> double fitnessFunction(Chromosome<T> chromosome, std::vector<T> result) {
  int numErrors = 0;
  for (int i = 0; i < chromosome.getChromosomeLength(); i++) {
    if (chromosome[i] != result[i])
      numErrors++;
  }
  return 1.0 / (1 + numErrors);
}

int main () {
  std::vector<int> genePool {0,1};
  Chromosome<int> c(genePool, 10);
  std::cout << c.getChromosomeLength() << std::endl;
  return 0;
}
