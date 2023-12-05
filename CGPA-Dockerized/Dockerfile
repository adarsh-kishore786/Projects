# the base OS
FROM alpine:3.18.5

# installing required packages
RUN apk add git
RUN apk add make
RUN apk add g++

# cloning into the work directory
RUN git clone https://github.com/adarsh-kishore786/Projects.git 
RUN cd Projects/CGPA
WORKDIR Projects/CGPA

# copying local files needed
COPY grades/ grades/

# time to test this!
CMD make
