package com.example.frontend

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class ViewModel : ViewModel() {

    // 1. State: The list of Todos
    // We use MutableStateFlow so we can update it, but expose it as StateFlow (read-only)
    private val _todos = MutableStateFlow<List<Todo>>(emptyList())
    val todos: StateFlow<List<Todo>> = _todos.asStateFlow()

    // 2. State: Is the app currently loading?
    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading.asStateFlow()

    init {
        // As soon as the ViewModel is created, fetch data from Rust
        fetchTodos()
    }

    /**
     * Gets the current list of todos from the Rust server
     */
    fun fetchTodos() {
        viewModelScope.launch {
            _isLoading.value = true
            try {
                // We call the 'instance' val we created in Client
                val result = Client.instance.getTodos()
                _todos.value = result
            } catch (e: Exception) {
                // In a real app, you'd show a Toast or Error message to the user
                e.printStackTrace()
            } finally {
                _isLoading.value = false
            }
        }
    }

    /**
     * Tells the Rust server to mark a todo as complete
     */
    fun toggleComplete(id: Int) {
        viewModelScope.launch {
            try {
                // This hits your GET /todos/{id}/complete route
                Client.instance.completeTodo(id)

                // After the server updates the CSV, we refresh our local list
                fetchTodos()
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    /**
     * Sends a new Todo to the Rust server
     */
    fun addNewTodo(taskDescription: String) {
        viewModelScope.launch {
            try {
                // Generate a simple ID (usually the backend handles this, but we'll send one)
                val newTodo = Todo(
                    id = (100..9999).random(),
                    task = taskDescription,
                    completed = false
                )

                Client.instance.addTodo(newTodo)
                fetchTodos() // Refresh list
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }
}
