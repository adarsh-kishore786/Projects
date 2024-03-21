#include <iostream>
#include <vector>

#include "chromosome.h"

int main () {
  std::vector<int> genePool {0,1};
  Chromosome<int> c(genePool, 10);
  std::cout << c.getChromosomeLength() << std::endl;
  return 0;
}
