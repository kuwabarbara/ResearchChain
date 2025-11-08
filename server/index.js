const express = require('express');
const bodyParser = require('body-parser');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
app.use(cors());
app.use(bodyParser.json());

const DB = path.join(__dirname, 'db.json');

function load() {
  if (!fs.existsSync(DB)) return { papers: [] };
  return JSON.parse(fs.readFileSync(DB, 'utf8'));
}
function save(data) {
  fs.writeFileSync(DB, JSON.stringify(data, null, 2), 'utf8');
}

app.post('/api/register', (req, res) => {
  const { title, author, hashHex } = req.body || {};
  if (!hashHex) return res.status(400).json({ error: 'hashHex required' });

  const db = load();
  const id = `${Date.now()}`;
  const item = {
    id,
    title: title || 'untitled',
    author: author || 'anonymous',
    hashHex,
    createdAt: new Date().toISOString(),
  };
  db.papers.push(item);
  save(db);
  res.json({ ok: true, id });
});

app.get('/api/latest', (req, res) => {
  const db = load();
  const item = db.papers.slice(-1)[0] || null;
  if (!item) return res.status(404).json({ error: 'empty' });
  res.json(item);
});

const PORT = 8787;
app.listen(PORT, () => {
  console.log(`Bridge server on http://localhost:${PORT}`);
});
