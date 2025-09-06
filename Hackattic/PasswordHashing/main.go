package main

import (
  "fmt"
  "log"
  "os"
  "net/http"
  "io/ioutil"
  "encoding/json"
  "crypto/sha256"

  "github.com/joho/godotenv"
)

const URL = "https://hackattic.com/challenges/password_hashing";

type Problem struct {
  Password string `json:"password"`
  Salt     string `json:"salt"`

  Pbkdf2   struct {
    Rounds int    `json:"rounds"`
    Hash   string `json:"hash"`
  } `json:"pbkdf2"`

  Scrypt   struct {
    N        int    `json:"N"`
    R        int    `json:"r"`
    P        int    `json:"p"`
    Buflen   int    `json:"buflen"`
    Control  string `json:"_control"`
  } `json:"scrypt"`
}

type Result struct {
  Sha256 string
  Hmac   string
  Pbkdf2 string
  Scrypt string
}

func GetProblem(accessToken string) Problem {
  response, err := http.Get(URL + "/problem?access_token=" + accessToken)

  if err != nil {
    fmt.Println(err.Error())
    os.Exit(1)
  }

  responseData, err := ioutil.ReadAll(response.Body)
  if err != nil {
    log.Fatal(err)
  }

  defer response.Body.Close()
  var problem Problem
  err = json.Unmarshal([]byte(responseData), &problem)
  if err != nil {
    log.Fatal(err)
  }

  return problem
}

func Process(problem Problem) {
  password := problem.Password
  sha := GetSha256(password)

  fmt.Println(sha)
}

func GetSha256(password string) string {
  h := sha256.New()

  h.Write([]byte(password))
  return fmt.Sprintf("%x", h.Sum(nil))
}

func main()  {
  err := godotenv.Load()
  if err != nil {
    log.Fatal("Could not find .env file!")
  }

  accessToken := os.Getenv("ACCESS_TOKEN")

  problem := GetProblem(accessToken)
  Process(problem)
}
