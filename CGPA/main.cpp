#include <iostream>
#include <string>

#include "course.h"
#include "semester.h"

int main() {
  auto semester = Semester();
  std::string baseUrl = "grades/5.txt";

  semester.inputCourses(baseUrl);
  std::cout << semester;
  return 0;
}
