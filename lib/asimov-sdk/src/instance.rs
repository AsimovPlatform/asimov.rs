// This is free and unencumbered software released into the public domain.

use crate::{
    flow::{FlowDefinition, LocalFlowDefinition},
    prelude::{vec, Box, Vec},
    BlockDefinition, LocalBlockDefinition, LocalModelManifest, ModelManifest, ModuleRegistration,
    StaticModuleRegistration,
};
use asimov_sys::{
    asiCreateInstance, asiDestroyInstance, asiEnumerateBlocks, asiEnumerateFlows,
    asiEnumerateModels, asiEnumerateModules, AsiBlockDefinition, AsiFlowDefinition, AsiInstance,
    AsiModelManifest, AsiModuleRegistration, AsiResult, ASI_NULL_HANDLE,
};
use core::ptr::{null, null_mut};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instance {
    handle: AsiInstance,
}

impl Instance {
    pub fn new() -> Result<Self, AsiResult> {
        let mut instance = Self {
            handle: ASI_NULL_HANDLE,
        };
        match unsafe { asiCreateInstance(null(), &mut instance.handle) } {
            AsiResult::ASI_SUCCESS => Ok(instance),
            error => Err(error),
        }
    }

    #[stability::unstable]
    pub fn blocks(&self) -> Result<Vec<Box<dyn BlockDefinition>>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateBlocks(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        let mut buffer = vec![AsiBlockDefinition::default(); count as _];
        match unsafe { asiEnumerateBlocks(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        Ok(buffer
            .into_iter()
            .map(|inner| Box::new(LocalBlockDefinition::new(inner)) as _)
            .collect())
    }

    #[stability::unstable]
    pub fn flows(&self) -> Result<Vec<Box<dyn FlowDefinition>>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateFlows(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        let mut buffer = vec![AsiFlowDefinition::default(); count as _];
        match unsafe { asiEnumerateFlows(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        Ok(buffer
            .into_iter()
            .map(|inner| Box::new(LocalFlowDefinition::new(inner)) as _)
            .collect())
    }

    #[stability::unstable]
    pub fn models(&self) -> Result<Vec<Box<dyn ModelManifest>>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateModels(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        let mut buffer = vec![AsiModelManifest::default(); count as _];
        match unsafe { asiEnumerateModels(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        Ok(buffer
            .into_iter()
            .map(|inner| Box::new(LocalModelManifest::new(inner)) as _)
            .collect())
    }

    #[stability::unstable]
    pub fn modules(&self) -> Result<Vec<Box<dyn ModuleRegistration>>, AsiResult> {
        let mut count: u32 = 0;
        match unsafe { asiEnumerateModules(self.handle, 0, &mut count, null_mut()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        let mut buffer = vec![AsiModuleRegistration::default(); count as _];
        match unsafe { asiEnumerateModules(self.handle, count, &mut count, buffer.as_mut_ptr()) } {
            AsiResult::ASI_SUCCESS => (),
            error => return Err(error),
        };

        Ok(buffer
            .into_iter()
            .map(|inner| Box::new(StaticModuleRegistration::new(inner)) as _)
            .collect())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        if self.handle == ASI_NULL_HANDLE {
            return;
        }
        match unsafe { asiDestroyInstance(self.handle) } {
            AsiResult::ASI_SUCCESS => self.handle = ASI_NULL_HANDLE,
            _ => unreachable!("instance should be destroyed"),
        }
    }
}
