// This is free and unencumbered software released into the public domain.

use crate::prelude::{fmt::Debug, Cow, Named, String};
use asimov_sys::AsiFlowExecution;

#[stability::unstable]
pub trait FlowExecution: Named + Debug {}

#[derive(Debug)]
pub(crate) struct LocalFlowExecution {
    inner: AsiFlowExecution,
}

impl LocalFlowExecution {
    #[allow(unused)]
    pub fn new(name: &str, pid: u64) -> Self {
        Self {
            inner: AsiFlowExecution::new(name, pid),
        }
    }
}

impl From<AsiFlowExecution> for LocalFlowExecution {
    fn from(inner: AsiFlowExecution) -> Self {
        Self { inner }
    }
}

impl Named for LocalFlowExecution {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl FlowExecution for LocalFlowExecution {}