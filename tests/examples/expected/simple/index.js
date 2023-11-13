const express = require("express");

const app = express();

app.use("/", root);

app.get("/", (req, res) => {
  res.send("Hello World!");
});

app.listen(3000, () => {
  console.log("app is listening on 3000");
});
