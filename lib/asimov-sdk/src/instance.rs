// This is free and unencumbered software released into the public domain.

use crate::{
    flow::FlowDefinition,
    prelude::{null, Box},
    BlockDefinition, BlockDefinitionIter, Error, FlowDefinitionIter, ModelManifest,
    ModelManifestIter, ModuleRegistration, ModuleRegistrationIter, Result,
};
use asimov_sys::{asiCreateInstance, asiDestroyInstance, AsiInstance, AsiResult, ASI_NULL_HANDLE};

#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instance {
    handle: AsiInstance,
}

impl Instance {
    pub fn new() -> Result<Self> {
        let mut instance = Self {
            handle: ASI_NULL_HANDLE,
        };
        match unsafe { asiCreateInstance(null(), &mut instance.handle) } {
            AsiResult::ASI_SUCCESS => Ok(instance),
            error => Err(error.try_into().unwrap()),
        }
    }

    #[stability::unstable]
    pub fn blocks(&self) -> Result<impl Iterator<Item = Box<dyn BlockDefinition>>> {
        BlockDefinitionIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn constructs(&self) -> Result<()> {
        todo!("Instance#constructs") // TODO
    }

    #[stability::unstable]
    pub fn flows(&self) -> Result<impl Iterator<Item = Box<dyn FlowDefinition>>> {
        FlowDefinitionIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn models(&self) -> Result<impl Iterator<Item = Box<dyn ModelManifest>>> {
        ModelManifestIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn modules(&self) -> Result<impl Iterator<Item = Box<dyn ModuleRegistration>>> {
        ModuleRegistrationIter::try_from(self.handle)
    }

    #[stability::unstable]
    pub fn schemas(&self) -> Result<()> {
        todo!("Instance#schemas") // TODO
    }

    #[stability::unstable]
    pub fn vaults(&self) -> Result<()> {
        todo!("Instance#vaults") // TODO
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
