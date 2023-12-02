#ifndef __COLLEGE_H_INCLUDED__
#define __COLLEGE_H_INCLUDED__

#include <string>
#include <vector>
#include "semester.h"

class College {
private:
  std::string collegeName;
  std::vector<Semester> semesters;

  void inputSemesters();

public:
  College(std::string& collegeName);

  std::string getCollegeName() const;
  std::vector<Semester> getSemesters() const;

  double getCGPA(int numSemesters) const;

  friend std::ostream& operator<<(std::ostream& os, const College& college);
};

#endif
