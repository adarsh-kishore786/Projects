import express from 'express';

const app = express();
const PORT = 3000;

app.get("/", (req, res) => {
  console.log("hello");
  res.send("hello");
});

app.listen(PORT, () => {
  console.log(`Server is listening on localhost:${PORT}`);
});
