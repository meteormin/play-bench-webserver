<?php

$todos = [];
$lastId = 0;

$method = $_SERVER['REQUEST_METHOD'];
$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

if ($path === '/todos') {
    if ($method === 'GET') {
        echo json_encode(array_values($todos));
    } elseif ($method === 'POST') {
        $input = json_decode(file_get_contents('php://input'), true);
        $lastId++;
        $todo = array_merge(['id' => $lastId], $input);
        $todos[$lastId] = $todo;
        http_response_code(201);
        echo json_encode($todo);
    }
} elseif (preg_match('/^\/todos\/(\d+)$/', $path, $matches)) {
    $id = (int)$matches[1];
    if ($method === 'GET') {
        if (!isset($todos[$id])) {
            http_response_code(404);
            echo json_encode(['error' => 'not found']);
        } else {
            echo json_encode($todos[$id]);
        }
    } elseif ($method === 'PUT') {
        if (!isset($todos[$id])) {
            http_response_code(404);
            echo json_encode(['error' => 'not found']);
        } else {
            $input = json_decode(file_get_contents('php://input'), true);
            $todo = array_merge(['id' => $id], $input);
            $todos[$id] = $todo;
            echo json_encode($todo);
        }
    } elseif ($method === 'DELETE') {
        unset($todos[$id]);
        http_response_code(204);
    }
}
