use std::sync::Mutex;

use tonic::transport::Server;

use hello_grpc::todo::{
    CreateTodoRequest,
    CreateTodoResponse, GetTodosResponse, todo_server::{Todo, TodoServer}, TodoItem,
};

#[derive(Debug, Default)]
pub struct TodoService {
    todos: Mutex<Vec<TodoItem>>,
}

#[tonic::async_trait]
impl Todo for TodoService {
    async fn get_todos(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<GetTodosResponse>, tonic::Status> {
        let message = GetTodosResponse {
            todos: self.todos.lock().unwrap().to_vec(),
        };

        Ok(tonic::Response::new(message))
    }

    async fn create_todo(
        &self,
        request: tonic::Request<CreateTodoRequest>,
    ) -> Result<tonic::Response<CreateTodoResponse>, tonic::Status> {
        let payload = request.into_inner();

        let todo_item = TodoItem {
            name: payload.name,
            description: payload.description,
            priority: payload.priority,
            completed: false,
        };

        self.todos.lock().unwrap().push(todo_item.clone());

        let message = CreateTodoResponse {
            todo: Some(todo_item),
            status: true,
        };

        Ok(tonic::Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let todo_service = TodoService::default();

    println!("server start at {:?}", addr);
    Server::builder()
        .add_service(TodoServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}