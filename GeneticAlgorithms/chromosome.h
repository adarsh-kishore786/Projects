#ifndef __CHROMOSOME_H_INCLUDED__
#define __CHROMOSOME_H_INCLUDED__

#include <iostream>
#include <vector>
#include <time.h>

template <typename T> class Chromosome {
private:
  std::vector<T> geneArray;
  int chromosomeLength;

public:
  Chromosome(const std::vector<T>, int);

  T& operator[](int);
  int getChromosomeLength();

  template <typename Y> friend std::ostream& operator<< (std::ostream&, const Chromosome<Y>&);
};

template <typename T> Chromosome<T>::Chromosome(std::vector<T> genePool, int chromosomeLength) {
  this->chromosomeLength = chromosomeLength;
  srand(time(NULL));

  for (int i = 0; i < chromosomeLength; i++) {
    this->geneArray.push_back(genePool[rand() % genePool.size()]);
  }
}

template <class T> T& Chromosome<T>::operator[](int index) {
  if (index >= this->chromosomeLength) {
    std::cout << "Error! Requesting gene beyond chromosome length!" << std::endl;
    exit(1);
  }
  return this->geneArray[index];
}

template <class T> int Chromosome<T>::getChromosomeLength() {
  return this->chromosomeLength;
}

template <class T> std::ostream& operator<<(std::ostream& os, const Chromosome<T>& chromosome) {
  os << "Chromosome ";
  for (auto gene : chromosome.geneArray) {
    os << gene;
  }
  os << std::endl;
  return os;
}

#endif
