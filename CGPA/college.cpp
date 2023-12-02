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

  int numSemesters = 0;
  std::cout << "Enter the number of semesters you have had: ";
  std::cin >> numSemesters;
  std::cout << "Assuming the semesters are in `grades/`\n";

  for (int i = 1; i <= numSemesters; i++) {
    auto sem = Semester();
    auto fileUrl = inputUrl + (char)(i+48);
    sem.inputCourses(fileUrl);

    semesters.push_back(sem);
  }
}

double College::getCGPA(int numSemesters) const {
  double currCGPA = 0;
  int totalCredits = 0;

  for (int i = 1; i <= numSemesters; i++) {
    auto sem = semesters[i-1];
    if (totalCredits == 0) {
      currCGPA = sem.getSGPA();
    } else {
      auto sgpa = sem.getSGPA();
      auto credits = sem.getSemCredits();

      currCGPA = (currCGPA * totalCredits + sgpa * credits) / (totalCredits + credits);
    }
    totalCredits += sem.getSemCredits();
  }
  return currCGPA;
}

std::ostream& operator<<(std::ostream& os, const College& college) {
  os << "College Name: " << college.getCollegeName() << "\n\n";
  auto semesters = college.getSemesters();

  for (int i = 0; i < semesters.size(); i++) {
    auto sem = semesters[i];
    os << sem << "\n";
    os << "CGPA till now: " << college.getCGPA(i+1) << "\n\n";
  }
  os << "Total CGPA: " << college.getCGPA(semesters.size()) << "\n";
  return os;
}
