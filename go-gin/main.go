package main

import (
	"net/http"
	"strconv"
	"sync"

	"github.com/gin-gonic/gin"
)

type Todo struct {
	ID        int    `json:"id"`
	Title     string `json:"title"`
	Completed bool   `json:"completed"`
}

var (
	todos = make(map[int]Todo)
	mu    sync.Mutex
	lastID = 0
)

func main() {
	gin.SetMode(gin.ReleaseMode)
	r := gin.Default()

	r.GET("/todos", func(c *gin.Context) {
		mu.Lock()
		defer mu.Unlock()
		res := make([]Todo, 0, len(todos))
		for _, v := range todos {
			res = append(res, v)
		}
		c.JSON(http.StatusOK, res)
	})

	r.POST("/todos", func(c *gin.Context) {
		var todo Todo
		if err := c.ShouldBindJSON(&todo); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		mu.Lock()
		lastID++
		todo.ID = lastID
		todos[todo.ID] = todo
		mu.Unlock()
		c.JSON(http.StatusCreated, todo)
	})

	r.GET("/todos/:id", func(c *gin.Context) {
		id, _ := strconv.Atoi(c.Param("id"))
		mu.Lock()
		todo, ok := todos[id]
		mu.Unlock()
		if !ok {
			c.JSON(http.StatusNotFound, gin.H{"error": "not found"})
			return
		}
		c.JSON(http.StatusOK, todo)
	})

	r.PUT("/todos/:id", func(c *gin.Context) {
		id, _ := strconv.Atoi(c.Param("id"))
		var todo Todo
		if err := c.ShouldBindJSON(&todo); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}
		mu.Lock()
		if _, ok := todos[id]; !ok {
			mu.Unlock()
			c.JSON(http.StatusNotFound, gin.H{"error": "not found"})
			return
		}
		todo.ID = id
		todos[id] = todo
		mu.Unlock()
		c.JSON(http.StatusOK, todo)
	})

	r.DELETE("/todos/:id", func(c *gin.Context) {
		id, _ := strconv.Atoi(c.Param("id"))
		mu.Lock()
		delete(todos, id)
		mu.Unlock()
		c.Status(http.StatusNoContent)
	})

	r.Run(":8080")
}
