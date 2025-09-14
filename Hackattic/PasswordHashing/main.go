package main

import (
  "crypto/sha256"
  "encoding/base64"
  "encoding/binary"
  "encoding/hex"
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
  salt := BytesAlpha(saltBytes)
  secretKey := ComputeBlockKeySha256(salt)
  password += salt

  fmt.Println(len(password))

  sha := GetSha256(password)
  hmac := GetHMACSha256(password, secretKey)
  pbkdf2 := GetPbkdf2(password, secretKey, problem.Pbkdf2.Rounds, 64)

  fmt.Println("SHA-256 : " + sha)
  fmt.Println("HMAC-256: " + hmac)
  fmt.Println("Pbkdf2  : " + pbkdf2)
}

func GetSha256(password string) string {
  h := sha256.New()

  h.Write([]byte(password))
  return BytesAlpha(h.Sum(nil))
}

func GetHMACSha256(password string, secretKey string) string {
  ipad := ByteXOR(secretKey, 54)
  opad := ByteXOR(secretKey, 92)

  message := GetSha256(opad + GetSha256(ipad + password))

  return message 
}

func GetPbkdf2(password string, secretKey string, c int, dkLen int) string {
  input := secretKey
  result := ""

  for i := range(dkLen / 64) {
    temp := secretKey + GetBigEndian(i)
    fmt.Println(temp)

    for _ = range(c) {
      input = GetHMACSha256(password, input)
      temp = FullXOR(temp, input)
    }

    result += temp
  }

  return result
}

func GetScrypt(password string, secretKey string, r int, p int, buflen int) string {
  stretchkedKey := GetPbkdf2(password, secretKey, 1, (p * r * 128) / 64)

  return stretchkedKey
}

func FullXOR(str1 string, str2 string) string {
  bytes1 := BytesInt(str1)
  bytes2 := BytesInt(str2)

  for i := range(len(bytes1)) {
    bytes1[i] ^= bytes2[i] 
  }

  return BytesAlpha(bytes1)
}

func ByteXOR(key string, val byte) string {
  keyBytes := BytesInt(key)

  for i := range(len(keyBytes)) {
    keyBytes[i] ^= val
  }

  return BytesAlpha(keyBytes)
}

func GetBigEndian(num int) string {
  buf := make([]byte, 8)

  binary.BigEndian.PutUint64(buf, uint64(num))
  
  return BytesAlpha(buf)
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

func BytesAlpha(bytes []byte) string {
  return fmt.Sprintf("%x", bytes)
}

func BytesInt(bytesAlpha string) []byte {
  decodeBytes, err := hex.DecodeString(bytesAlpha)
  if err != nil {
    log.Fatal(err)
  }

  return decodeBytes
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
