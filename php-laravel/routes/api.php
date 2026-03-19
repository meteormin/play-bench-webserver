<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;

$todos = [];
$lastId = 0;

Route::get('/todos', function () use (&$todos) {
    return response()->json(array_values($todos));
});

Route::post('/todos', function (Request $request) use (&$todos, &$lastId) {
    $lastId++;
    $todo = array_merge(['id' => $lastId], $request->all());
    $todos[$lastId] = $todo;
    return response()->json($todo, 201);
});

Route::get('/todos/{id}', function ($id) use (&$todos) {
    if (!isset($todos[$id])) return response()->json(['error' => 'not found'], 404);
    return response()->json($todos[$id]);
});

Route::put('/todos/{id}', function (Request $request, $id) use (&$todos) {
    if (!isset($todos[$id])) return response()->json(['error' => 'not found'], 404);
    $todo = array_merge(['id' => (int)$id], $request->all());
    $todos[$id] = $todo;
    return response()->json($todo);
});

Route::delete('/todos/{id}', function ($id) use (&$todos) {
    unset($todos[$id]);
    return response('', 204);
});
