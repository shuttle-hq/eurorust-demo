const todoInput = document.getElementById("todoInput");
const todoList = document.getElementById("todoList");

async function fetchTodos() {
  try {
    const todoIds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    todoList.innerHTML = "";
    const first10TodoIds = todoIds.slice(0, 10);
    for (const todoId of first10TodoIds) {
      const detailedTodo = await fetchTodoById(todoId);
      if (detailedTodo) {
        const listItem = document.createElement("li");
        listItem.textContent = detailedTodo.note;
        todoList.appendChild(listItem);
      }
    }
  } catch (error) {
    console.error("Error fetching todos:", error);
  }
}

async function fetchTodoById(id) {
  try {
    const response = await fetch(`/todo/${id}`);
    const todo = await response.json();
    return todo;
  } catch (error) {
    console.error(`Error fetching todo with ID ${id}:`, error);
    return null;
  }
}

async function addTodo() {
  const note = todoInput.value.trim();
  if (note === "") return;

  try {
    const response = await fetch("/todo", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ note }),
    });

    if (response.ok) {
      todoInput.value = "";
      fetchTodos();
    } else {
      console.error("Failed to add todo.");
    }
  } catch (error) {
    console.error("Error adding todo:", error);
  }
}

fetchTodos();
