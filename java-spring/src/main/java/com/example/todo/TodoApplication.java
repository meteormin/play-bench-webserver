package com.example.todo;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.*;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;

@SpringBootApplication
@RestController
@RequestMapping("/todos")
public class TodoApplication {

    public static void main(String[] args) {
        SpringApplication.run(TodoApplication.class, args);
    }

    private final Map<Integer, Todo> todos = new ConcurrentHashMap<>();
    private final AtomicInteger lastId = new AtomicInteger();

    @GetMapping
    public List<Todo> list() {
        return new ArrayList<>(todos.values());
    }

    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public Todo create(@RequestBody Todo todo) {
        int id = lastId.incrementAndGet();
        todo.setId(id);
        todos.put(id, todo);
        return todo;
    }

    @GetMapping("/{id}")
    public ResponseEntity<Todo> get(@PathVariable int id) {
        Todo todo = todos.get(id);
        if (todo == null) return ResponseEntity.notFound().build();
        return ResponseEntity.ok(todo);
    }

    @PutMapping("/{id}")
    public ResponseEntity<Todo> update(@PathVariable int id, @RequestBody Todo todo) {
        if (!todos.containsKey(id)) return ResponseEntity.notFound().build();
        todo.setId(id);
        todos.put(id, todo);
        return ResponseEntity.ok(todo);
    }

    @DeleteMapping("/{id}")
    @ResponseStatus(HttpStatus.NO_CONTENT)
    public void delete(@PathVariable int id) {
        todos.remove(id);
    }

    public static class Todo {
        private int id;
        private String title;
        private boolean completed;

        public Todo() {}
        public int getId() { return id; }
        public void setId(int id) { this.id = id; }
        public String getTitle() { return title; }
        public void setTitle(String title) { this.title = title; }
        public boolean isCompleted() { return completed; }
        public void setCompleted(boolean completed) { this.completed = completed; }
    }
}
