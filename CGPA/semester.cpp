#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <algorithm>
#include <numeric>

#include "semester.h"

int getSemNumber(std::string& fileUrl);

Semester::Semester() {
  this->semNumber = 0;
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

void Semester::inputCourses(std::string& fileUrl) {
  semNumber = ::getSemNumber(fileUrl);

  std::ifstream semDetails(fileUrl);
  std::string courseDetails;

  if (semDetails.is_open()) {
    while (std::getline(semDetails, courseDetails)) {
      std::string courseCode = courseDetails.substr(0, courseDetails.find(','));

      std::string otherDetails = courseDetails.substr(courseDetails.find(',')+1);
      int credits = stoi(otherDetails.substr(0, otherDetails.find(',')));

      double grade = std::stod(otherDetails.substr(otherDetails.find(',')+1));
      if (grade < 0 || grade > 10) {
        std::cout << "The input grades in " << semNumber << " are not between 0 and 10! Aborting...\n";
        exit(0);
      }
      addCourse(Course(courseCode, credits, grade));
    }
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

  os << "----------------------------\n";
  os << title << credits << courses;
  for (auto& course : semester.getSemCourses()) {
    os << course << "\n";
  }
  os << "SGPA: " << semester.getSGPA() << "\n";
  os << "----------------------------\n";
  return os;
}

int getSemNumber(std::string& fileUrl) {
  auto indexOfSlash = fileUrl.find_last_of('/');
  if (indexOfSlash == std::string::npos)
    indexOfSlash = 0;
  else 
    indexOfSlash += 1;

  auto fileNameAndExtension = fileUrl.substr(indexOfSlash);
  auto indexOfDot = fileNameAndExtension.find('.');
  if (indexOfDot == std::string::npos)
    indexOfDot = fileNameAndExtension.size();

  auto fileName = fileNameAndExtension.substr(0, indexOfDot);
  return stoi(fileName);
}
