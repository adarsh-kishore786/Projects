package com.example.frontend

import retrofit2.http.*

interface Interface {
    @GET("todos")
    suspend fun getTodos(): List<Todo>

    @POST("todos")
    suspend fun addTodo(@Body todo: Todo): Todo

    @GET("todos/{id}/complete")
    suspend fun completeTodo(@Path("id") id: Int): String
}