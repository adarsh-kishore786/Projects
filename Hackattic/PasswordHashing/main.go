package main

import (
  "fmt"
  "log"
  "os"
  "net/http"
  "io/ioutil"

  "github.com/joho/godotenv"
)

const URL = "https://hackattic.com/challenges/password_hashing";

func GetProblem(accessToken string) string {
  response, err := http.Get(URL + "/problem?access_token=" + accessToken)

  if err != nil {
    fmt.Println(err.Error())
    os.Exit(1)
  }

  responseData, err := ioutil.ReadAll(response.Body)
  if err != nil {
    log.Fatal(err)
  }

  return string(responseData)
}

func main()  {
  err := godotenv.Load()
  if err != nil {
    log.Fatal("Could not find .env file!")
  }

  accessToken := os.Getenv("ACCESS_TOKEN")

  fmt.Println(GetProblem(accessToken))
}
