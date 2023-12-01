#ifndef __SEMESTER_H_INCLUDED__
#define __SEMESTER_H_INCLUDED__

#include <vector>
#include "course.h"

class Semester {

private:
  int semNumber;
  int semCredits;
  std::vector<Course> semCourses;
  double SGPA;

public:
  Semester(int semNumber);

  int getSemNumber() const;
  int getSemCredits() const;
  std::vector<Course> getSemCourses() const;

  int getNumCourses() const;
  double getSGPA() const;

  void addCourse(const Course& course);
  void inputCourses();

  friend std::ostream& operator<<(std::ostream& os, const Semester& semester);
};

#endif
