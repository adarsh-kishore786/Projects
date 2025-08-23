#ifndef __GAOPERATORS_H_INCLUDED__
#define __GAOPERATORS_H_INCLUDED__

#include <vector>
#include <functional>

class GAOperators {
private:
  template <typename T> std::function<int(Chromosome<T>, std::vector<T>)>;

public:
  template <typename T> GAOperators<T>(std::function<int(Chromosome<T>, std::vector<T>)>);
  template <typename T> int calculateFitnessOfPopulation(std::vector<Chromosome<T>>);
  template <typename T> void crossOver(std::vector<Chromosome<T>>);
  template <typename T> void mutate(std::vector<Chromosome<T>>);
}

#endif
