import express from "express";
import cors from "cors";

const app = express();
const PORT = 8000;

app.use(cors());
app.use(express.json());

app.get("/api/data", (_req, res) => {
  res.json({
    message: "Hello from backend!",
    time: new Date().toISOString(),
  });
});

app.get("/api/csv", (_req, res) => {
  const csv = `name,age,city
              John,25,Prague
              Anna,30,Brno
              Vojtech,17,Most`;

  res.setHeader("Content-Type", "text/csv");
  res.setHeader("Content-Disposition", "attachment; filename=data.csv");

  res.send(csv);
});

app.listen(PORT, () => {
  console.log(`Backend running on http://localhost:${PORT}`);
});
