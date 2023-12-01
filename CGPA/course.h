#ifndef __COURSE_H_INCLUDED__
#define __COURSE_H_INCLUDED__

#include <string>

class Course {

private:
  std::string courseCode;
  int credits;
  double grade;

public:
  Course(std::string courseCode, int credits, double grade);

  std::string getCourseCode() const;
  int getCredits() const;
  double getGrade() const;

  friend std::ostream& operator<<(std::ostream& os, const Course& course);
};

#endif
