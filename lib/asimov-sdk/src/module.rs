// This is free and unencumbered software released into the public domain.

use crate::{
    prelude::{fmt::Debug, Cow, Result, String},
    Named,
};
use asimov_sys::{AsiInstance, AsiModuleRegistration};

#[stability::unstable]
pub trait ModuleRegistration: Named + Debug {
    fn is_enabled(&self) -> bool {
        true
    }

    fn enable(&mut self) -> Result<bool, ()>;

    fn disable(&mut self) -> Result<bool, ()>;
}

#[derive(Debug)]
pub(crate) struct StaticModuleRegistration {
    #[allow(unused)]
    instance: AsiInstance,
    inner: AsiModuleRegistration,
}

impl StaticModuleRegistration {
    pub fn new(instance: AsiInstance, inner: AsiModuleRegistration) -> Self {
        Self { instance, inner }
    }
}

impl Named for StaticModuleRegistration {
    fn name(&self) -> Cow<str> {
        self.inner.name_lossy()
    }
}

impl ModuleRegistration for StaticModuleRegistration {
    fn is_enabled(&self) -> bool {
        true
    }

    fn enable(&mut self) -> Result<bool, ()> {
        Ok(false)
    }

    fn disable(&mut self) -> Result<bool, ()> {
        Ok(false)
    }
}
