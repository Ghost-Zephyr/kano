use std::sync::Arc;

use colored::{ColoredString, Colorize};
use vulkano_win::{self, VkSurfaceBuild};
use vulkano::{
  instance::{
    InstanceCreateInfo, Instance,
  }
};

pub struct Kano {
  instance: Arc<Instance>,
}

#[inline]
fn error() -> String {
  "ERROR".bold().red().to_string()
}

impl Kano {

  pub fn new() -> Result<Self, String> {
    let required_extensions = vulkano_win::required_extensions();
    dbg!(required_extensions);

    let instance = match Instance::new(InstanceCreateInfo {
      enabled_extensions: required_extensions,
      enumerate_portability: true,
      ..Default::default()
    }) {
      Err(err) => {
        eprintln!("{} during vulkan instance creation \"{}\"", error(), err);
        return Err(format!("{} Unable to create a vulkan instance", error()));
      },
      Ok(v) => v,
    };

    Ok(Self {
      instance,
    })
  }
}
