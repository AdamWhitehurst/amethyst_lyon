//! Description:
//!
//! A custom render plugin for meshes created with Lyon.
//!
//! Copyright © 2020 Benedict Gaster. All rights reserved.
//!
pub mod pass;
pub mod utils;

use amethyst::{
    core::ecs::{DispatcherBuilder, World},
    prelude::*,
    renderer::{
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        rendy::{factory::Factory, graph::render::RenderGroupDesc},
        types::Backend,
    },
};

use amethyst::Error;

use crate::pass::DrawLyonDesc;
use crate::utils::{ActiveMesh, Mesh};

#[derive(Default, Debug)]
pub struct RenderLyon {}

/// Render plugin for Lyon tessellator
impl<B: Backend> RenderPlugin<B> for RenderLyon {
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Add the required components to the world ECS
        world.register::<Mesh>();
        world.register::<ActiveMesh>();
        world.insert(ActiveMesh::default());
        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> Result<(), Error> {
        plan.extend_target(Target::Main, |ctx| {
            // Add our Description
            ctx.add(RenderOrder::Transparent, DrawLyonDesc::new().builder())?;
            Ok(())
        });
        Ok(())
    }
}
