<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;

class TodoController extends Controller
{
    private function getTodos()
    {
        return Cache::get('todos', []);
    }

    private function saveTodos($todos)
    {
        Cache::forever('todos', $todos);
    }

    public function index()
    {
        return response()->json(array_values($this->getTodos()));
    }

    public function store(Request $request)
    {
        $todos = $this->getTodos();
        $lastId = Cache::get('last_id', 0) + 1;
        
        $todo = [
            'id' => $lastId,
            'title' => $request->input('title'),
            'completed' => $request->input('completed', false),
        ];
        
        $todos[$lastId] = $todo;
        $this->saveTodos($todos);
        Cache::forever('last_id', $lastId);
        
        return response()->json($todo, 201);
    }

    public function show($id)
    {
        $todos = $this->getTodos();
        if (!isset($todos[$id])) {
            return response()->json(['error' => 'not found'], 404);
        }
        return response()->json($todos[$id]);
    }

    public function update(Request $request, $id)
    {
        $todos = $this->getTodos();
        if (!isset($todos[$id])) {
            return response()->json(['error' => 'not found'], 404);
        }
        
        $todos[$id] = array_merge($todos[$id], $request->only(['title', 'completed']));
        $this->saveTodos($todos);
        
        return response()->json($todos[$id]);
    }

    public function destroy($id)
    {
        $todos = $this->getTodos();
        unset($todos[$id]);
        $this->saveTodos($todos);
        
        return response(null, 204);
    }
}
