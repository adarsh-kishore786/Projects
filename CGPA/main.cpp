#include <iostream>
#include <string>

#include "college.h"

int main() {
  std::string collegeName = "NITK";
  auto college = College(collegeName);
  std::cout << college;

  return 0;
}
