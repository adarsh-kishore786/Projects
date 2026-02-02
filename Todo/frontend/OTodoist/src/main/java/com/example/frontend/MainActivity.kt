package com.example.frontend

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.viewModels
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.example.frontend.ui.theme.FrontendTheme

class MainActivity : ComponentActivity() {
    // 1. Initialize the ViewModel using the 'by viewModels()' delegate
    private val viewModel: ViewModel by viewModels()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            FrontendTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    // 2. Call the TodoScreen instead of Greeting
                    TodoScreen(
                        viewModel = viewModel,
                        modifier = Modifier.padding(innerPadding)
                    )
                }
            }
        }
    }
}

@Composable
fun TodoScreen(viewModel: ViewModel, modifier: Modifier = Modifier) {
    // 3. Observe the data from our ViewModel
    val todos by viewModel.todos.collectAsState()
    val isLoading by viewModel.isLoading.collectAsState()

    var newTaskText by remember { mutableStateOf("") }

    Column(modifier = modifier.padding(16.dp)) {
        Text(text = "Rust Todo App", style = MaterialTheme.typography.headlineMedium)

        Spacer(modifier = Modifier.height(8.dp))

        // Input Field and Add Button
        Row(verticalAlignment = Alignment.CenterVertically) {
            TextField(
                value = newTaskText,
                onValueChange = { newTaskText = it },
                modifier = Modifier.weight(1f),
                placeholder = { Text("New task...") },
                singleLine = true
            )
            Spacer(modifier = Modifier.width(8.dp))
            Button(onClick = {
                if (newTaskText.isNotBlank()) {
                    viewModel.addNewTodo(newTaskText)
                    newTaskText = ""
                }
            }) {
                Text("Add")
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        if (isLoading) {
            CircularProgressIndicator(modifier = Modifier.align(Alignment.CenterHorizontally))
        }

        // 4. The scrollable list of items
        LazyColumn(modifier = Modifier.fillMaxSize()) {
            items(todos) { todo ->
                TodoItem(
                    todo = todo,
                    onComplete = { viewModel.toggleComplete(todo.id) }
                )
            }
        }
    }
}

@Composable
fun TodoItem(todo: Todo, onComplete: () -> Unit) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 4.dp),
        colors = CardDefaults.cardColors(
            containerColor = if (todo.completed) Color(0xFFE8F5E9) else MaterialTheme.colorScheme.surfaceVariant
        )
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(text = todo.task, style = MaterialTheme.typography.bodyLarge)
                Text(
                    text = if (todo.completed) "Done" else "Pending",
                    style = MaterialTheme.typography.labelSmall,
                    color = if (todo.completed) Color(0xFF2E7D32) else Color.Gray
                )
            }
            if (!todo.completed) {
                IconButton(onClick = onComplete) {
                    Icon(imageVector = Icons.Default.Check, contentDescription = "Complete")
                }
            }
        }
    }
}
