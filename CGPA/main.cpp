#include <iostream>

#include "course.h"
#include "semester.h"

int main() {
  auto semester = Semester(7);

  semester.inputCourses();
  std::cout << semester;
  return 0;
}
