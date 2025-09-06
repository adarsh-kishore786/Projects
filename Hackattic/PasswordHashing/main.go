package main

import (
  "crypto/sha256"
  "encoding/base64"
  "encoding/json"
  "fmt"
  "io/ioutil"
  "log"
  "net/http"
  "os"

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
  saltEncoded := problem.Salt

  saltBytes, err := base64.StdEncoding.DecodeString(saltEncoded)
  if err != nil {
    log.Fatal(err)
  }
  salt := fmt.Sprintf("%x", saltBytes)
  password += salt
  fmt.Println(problem)

  sha := GetSha256(password)
  hmac := GettHMACSha256(password, salt)

  fmt.Println(salt)
  fmt.Println(sha)
  fmt.Println(hmac)
}

func GetSha256(password string) string {
  h := sha256.New()

  h.Write([]byte(password))
  return fmt.Sprintf("%x", h.Sum(nil))
}

func GettHMACSha256(password string, secretKey string) string {
  blockKey := ComputeBlockKeySha256(secretKey)

  return blockKey 
}

func FullXOR(key string, val byte) string {
  // keyBytes := []byte(key)
  // valBytes := make([]byte, 64)

  return key
}

func ComputeBlockKeySha256(key string) string {
  l := len(key)

  var blockKey string
  blockSize := 64

  if l > blockSize {
    blockKey = GetSha256(key)
  } else if (l < blockSize) {
    blockKey = key
    for _ = range(blockSize-l) {
      blockKey += "0"
    }
  }

  return blockKey
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
