#include <vector>
#include <string>
#include <algorithm>
#include <numeric>

#include "semester.h"

Semester::Semester(int semNumber) {
  this->semNumber = semNumber;
  this->semCredits = 0;
  this->semCourses = {};
}

int Semester::getSemCredits() const {
  return semCredits;
}

int Semester::getSemNumber() const {
  return semNumber;
}

int Semester::getNumCourses() const {
  return semCourses.size();
}

std::vector<Course> Semester::getSemCourses() const {
  return semCourses;
}

void Semester::addCourse(const Course& course) {
  semCourses.push_back(course);
  semCredits += course.getCredits();
}

void Semester::inputCourses() {
  int n_courses;

  std::cout << "Enter the number of courses: ";
  std::cin >> n_courses;

  std::cout << "\nEnter the course details:\n";
  for (int i = 0; i < n_courses; i++) {
    std::string courseCode;
    int credits;
    double grade = -1;

    std::cout << "Course " << i+1 << ":\n";
    std::cout << "Course Code: ";
    std::cin >> courseCode;

    std::cout << "Credits: ";
    std::cin >> credits;

    while (grade < 0 || grade > 10) {
      std::cout << "Grade: ";
      std::cin >> grade;

      if (grade < 0 || grade > 10) {
        std::cout << "Grade must be between 0 and 10! Try again...\n\n";
        continue;
      }
      std::cout << "\n";
    }

    addCourse(Course(courseCode, credits, grade));
  }
}

double Semester::getSGPA() const {
  std::vector<double> equivalentGrades(semCourses.size());
  auto getEquivalentGrade = [&](Course c) { 
    return c.getGrade() * c.getCredits();
  };

  std::transform(semCourses.begin(), semCourses.end(), equivalentGrades.begin(), getEquivalentGrade);
  return std::accumulate(equivalentGrades.begin(), equivalentGrades.end(), 0.0) / semCredits;
}

std::ostream& operator<<(std::ostream& os, const Semester& semester) {
  std::string title = "SEMESTER " + std::to_string(semester.getSemNumber()) + "\n\n";
  std::string credits = "Total Credits: " + std::to_string(semester.getSemCredits()) + "\n";
  std::string courses = "Total Courses: " + std::to_string(semester.getNumCourses()) + "\n\n";

  os << title << credits << courses;
  for (auto& course : semester.getSemCourses()) {
    os << course << "\n";
  }
  os << "SGPA: " << semester.getSGPA() << "\n";
  return os;
}
