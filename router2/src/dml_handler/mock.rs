use std::{collections::VecDeque, sync::Arc};

use async_trait::async_trait;
use data_types::DatabaseName;

use hashbrown::HashMap;
use mutable_batch::MutableBatch;
use mutable_batch_lp::PayloadStatistics;
use parking_lot::Mutex;
use predicate::delete_predicate::HttpDeleteRequest;
use trace::ctx::SpanContext;

use super::{DmlError, DmlHandler};

#[derive(Debug, Clone)]
pub enum MockDmlHandlerCall {
    Write {
        namespace: String,
        batches: HashMap<String, MutableBatch>,
        payload_stats: PayloadStatistics,
        body_len: usize,
    },
    Delete(HttpDeleteRequest),
}

#[derive(Debug, Default)]
struct Inner {
    calls: Vec<MockDmlHandlerCall>,
    write_return: VecDeque<Result<(), DmlError>>,
    delete_return: VecDeque<Result<(), DmlError>>,
}

impl Inner {
    fn record_call(&mut self, call: MockDmlHandlerCall) {
        self.calls.push(call);
    }
}

#[derive(Debug, Default)]
pub struct MockDmlHandler(Mutex<Inner>);

impl MockDmlHandler {
    pub fn with_write_return(self, ret: impl Into<VecDeque<Result<(), DmlError>>>) -> Self {
        self.0.lock().write_return = ret.into();
        self
    }

    pub fn with_delete_return(self, ret: impl Into<VecDeque<Result<(), DmlError>>>) -> Self {
        self.0.lock().delete_return = ret.into();
        self
    }

    pub fn calls(&self) -> Vec<MockDmlHandlerCall> {
        self.0.lock().calls.clone()
    }
}

/// Mock helper to record a call and return the pre-configured value.
///
/// Pushes `$call` to call record, popping `self.state.$return` and returning it
/// to the caller. If no value exists, the pop attempt causes a panic.
macro_rules! record_and_return {
    ($self:ident, $call:expr, $return:ident) => {{
        let mut guard = $self.0.lock();
        guard.record_call($call);
        guard.$return.pop_front().expect("no mock value to return")
    }};
}

#[async_trait]
impl DmlHandler for Arc<MockDmlHandler> {
    async fn write(
        &self,
        namespace: DatabaseName<'_>,
        batches: HashMap<String, MutableBatch>,
        payload_stats: PayloadStatistics,
        body_len: usize,
        _span_ctx: Option<SpanContext>,
    ) -> Result<(), DmlError> {
        record_and_return!(
            self,
            MockDmlHandlerCall::Write {
                namespace: namespace.into(),
                batches,
                payload_stats,
                body_len,
            },
            write_return
        )
    }

    async fn delete(
        &self,
        delete: HttpDeleteRequest,
        _span_ctx: Option<SpanContext>,
    ) -> Result<(), DmlError> {
        record_and_return!(self, MockDmlHandlerCall::Delete(delete), delete_return)
    }
}
