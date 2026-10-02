//! Execution sub-system: DAG, error taxonomy, metrics, fan-out, and the 12-stage engine.

pub mod dag;
pub mod engine;
pub mod error;
pub mod fanout;
pub mod metrics;
pub mod trace;

pub use engine::{ExecutionEngine, ExecutionResult, Intent, IntentSignature, Outcome, TaskResult};
pub use dag::{Task, TaskDAG};
pub use error::ExecutionError;
pub use fanout::{FanoutJob, FanoutOutcome, JobError, JobOk, PartialFailure, TaskTimeout, run_fanout};
