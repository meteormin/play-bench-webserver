const express = require('express');
const app = express();
app.use(express.json());

let todos = [];
let lastId = 0;

app.get('/todos', (req, res) => {
    res.json(todos);
});

app.post('/todos', (req, res) => {
    const todo = { id: ++lastId, ...req.body };
    todos.push(todo);
    res.status(201).json(todo);
});

app.get('/todos/:id', (req, res) => {
    const todo = todos.find(t => t.id === parseInt(req.params.id));
    if (!todo) return res.status(404).json({ error: 'not found' });
    res.json(todo);
});

app.put('/todos/:id', (req, res) => {
    const index = todos.findIndex(t => t.id === parseInt(req.params.id));
    if (index === -1) return res.status(404).json({ error: 'not found' });
    todos[index] = { id: parseInt(req.params.id), ...req.body };
    res.json(todos[index]);
});

app.delete('/todos/:id', (req, res) => {
    todos = todos.filter(t => t.id !== parseInt(req.params.id));
    res.status(204).send();
});

app.listen(8080, () => {
    console.log('Express TODO API running on port 8080');
});
