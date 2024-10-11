#![no_std]
use sails_rs::{
    prelude::*,
    gstd::msg
};

pub mod service;
pub mod states;

use service::{
    ping_pong_service::PingService,
    query_service::QueryService
};

// Ping program struct
#[derive(Default)]
pub struct PingProgram;


// Program of the contract
#[program]  
impl PingProgram {
    pub fn new() -> Self {
        PingService::seed(msg::source());

        Self
    }

    #[route("Ping")]
    pub fn ping_svc(&self) -> PingService {
        PingService::new()
    }

    #[route("Query")]
    pub fn query_svc(&self) -> QueryService {
        QueryService::new()
    }
}