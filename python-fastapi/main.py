from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Optional

app = FastAPI()

class Todo(BaseModel):
    id: Optional[int] = None
    title: str
    completed: bool = False

todos = {}
last_id = 0

@app.get("/todos", response_model=List[Todo])
def get_todos():
    return list(todos.values())

@app.post("/todos", response_model=Todo, status_code=201)
def create_todo(todo: Todo):
    global last_id
    last_id += 1
    todo.id = last_id
    todos[last_id] = todo
    return todo

@app.get("/todos/{todo_id}", response_model=Todo)
def get_todo(todo_id: int):
    if todo_id not in todos:
        raise HTTPException(status_code=404, detail="not found")
    return todos[todo_id]

@app.put("/todos/{todo_id}", response_model=Todo)
def update_todo(todo_id: int, todo: Todo):
    if todo_id not in todos:
        raise HTTPException(status_code=404, detail="not found")
    todo.id = todo_id
    todos[todo_id] = todo
    return todo

@app.delete("/todos/{todo_id}", status_code=204)
def delete_todo(todo_id: int):
    if todo_id in todos:
        del todos[todo_id]
    return
