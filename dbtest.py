# This python is script is to fill the database with random data from [jsonplaceholder](https://jsonplaceholder.typicode.com/) API

import sqlite3
import requests

conn = sqlite3.connect("todos.db")
cur = conn.cursor()

cur.execute(
    """CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            label TEXT NOT NULL,
            completed BOOLEAN NOT NULL CHECK (completed IN (0, 1))
        )"""
)
conn.commit()

resp = requests.get("https://jsonplaceholder.typicode.com/todos/")
todos = resp.json()
todos = [(x["title"], 1 if x["completed"] else 0) for x in todos]

res = cur.executemany("INSERT INTO todos (label, completed) values (?, ?)", todos)
conn.commit()

res = cur.execute("SELECT * from todos")
for todo in res.fetchall():
    print(f"Added: {todo}")

conn.close()
