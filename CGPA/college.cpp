#include <iostream>
#include <string>
#include <vector>

#include "college.h"
#include "semester.h"

College::College(std::string& collegeName) {
  this->collegeName = collegeName;
  inputSemesters();
}

std::string College::getCollegeName() const {
  return collegeName;
}

std::vector<Semester> College::getSemesters() const {
  return semesters;
}

void College::inputSemesters() {
  std::string inputUrl = "grades/";
  int numSemesters = 5;

  for (int i = 1; i <= numSemesters; i++) {
    auto sem = Semester();
    auto fileUrl = inputUrl + (char)(i+48);
    sem.inputCourses(fileUrl);

    semesters.push_back(sem);
  }
}

std::ostream& operator<<(std::ostream& os, const College& college) {
  os << "College Name: " << college.getCollegeName() << "\n\n";

  for (auto& sem : college.getSemesters()) {
    os << sem << "\n";
  }
  return os;
}
