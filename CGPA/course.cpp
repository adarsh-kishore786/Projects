#include <string>
#include <iomanip>

#include "course.h"

Course::Course(std::string courseCode, int credits, double grade) {
  this->courseCode = courseCode;
  this->credits = credits;
  this->grade = grade;
}

std::string Course::getCourseCode() const {
  return courseCode;
}

int Course::getCredits() const {
  return credits;
}

double Course::getGrade() const {
  return grade;
}

std::ostream& operator<<(std::ostream& os, const Course& course) {
  auto courseCode = "Course " + course.getCourseCode() + "\n";
  auto credits = "Credits: " + std::to_string(course.getCredits()) + "\n";

  return os << courseCode << credits << "Grade: " << std::setprecision(3) << course.getGrade() << "\n";
}
