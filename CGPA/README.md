# CGPA Calculator

A program written in C++ which calculates your CGPA, given all the data of each semester.

## How To Use
Clone the repository and `cd` into the `CGPA` directory.
```
git clone https://github.com/adarsh-kishore786/Projects
cd CGPA
```
Semester files have to be stored in a directory called `CGPA/grades`, in the format `1`, `2`, and so on for each semester.

Each semester is stored as a csv file, with three entries per row, stored as:
```
Course Code,Credits,Grade
```
For example, if one of your courses is IT300 worth 4 credits, and you got 8 CGPA in it, and it was in the $5^{th}$ semester,
then it will be stored in the file `CGPA/grades/5` as:
```
IT300,4,8
```
After doing this for each semester, simply run the `Makefile` in `CGPA` by typing `make` in the directory containing the 
`Makefile`.
