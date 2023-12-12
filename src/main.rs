use tonic::{transport::Server, Request, Response, Status};

pub mod calculator {
    tonic::include_proto!("calculator");
}

use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::{CalculationSubjects, CalculationResult, Empty, Message};

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn hello(&self, _request: Request<Empty>) -> Result <Response<Message>, Status>
    {
        let result = Message { message: "Hello from Rust GRPC!".into() };
        Ok(Response::new(result))
    }

    async fn add(&self, request: Request<CalculationSubjects>) -> Result <Response<CalculationResult>, Status>
    {
        let req = request.into_inner();
        let result = CalculationResult { value: req.a + req.b };
        Ok(Response::new(result))
    }

    async fn subtract(&self, request: Request<CalculationSubjects>) -> Result <Response<CalculationResult>, Status>
    {
        let req = request.into_inner();
        let result = CalculationResult { value: req.a - req.b };
        Ok(Response::new(result))
    }

    async fn multiply(&self, request: Request<CalculationSubjects>) -> Result <Response<CalculationResult>, Status>
    {
        let req = request.into_inner();
        let result = CalculationResult { value: req.a * req.b };
        Ok(Response::new(result))
    }

    async fn divide(&self, request: Request<CalculationSubjects>) -> Result <Response<CalculationResult>, Status>
    {
        let req = request.into_inner();
        let result = CalculationResult { value: req.a / req.b };
        Ok(Response::new(result))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!("calculator_descriptor"))
        .build()?;

    let calculator_service = CalculatorService::default();

    Server::builder()
        .add_service(reflection_server)
        .add_service(CalculatorServer::new(calculator_service))
        .serve("0.0.0.0:3000".parse()?)
        .await?;

    Ok(())
}
