# CGPA-Dockerized

A dockerized version of the CGPA Application, which is available in this repository.

## How To Run
For example, suppose you want to build an image named `cgpa` out of this, then you will have to 
do the following:
```
docker image build -t cgpa .
docker container run cgpa
```
That's it! Everything is handled by the `Dockerfile`

## Change the data files
I have included some sample grade files in the repository. You can change those
into your own data files and build the image again.
