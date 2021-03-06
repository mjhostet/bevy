use crate::TypeRegistry;
use bevy_app::AppBuilder;
use bevy_ecs::{Component, FromResources};
use bevy_property::{DeserializeProperty, Properties, Property};

pub trait RegisterType {
    fn register_component<T>(&mut self) -> &mut Self
    where
        T: Properties + DeserializeProperty + Component + FromResources;
    fn register_properties<T>(&mut self) -> &mut Self
    where
        T: Properties + DeserializeProperty + FromResources;
    fn register_property<T>(&mut self) -> &mut Self
    where
        T: Property + DeserializeProperty;
}

impl RegisterType for AppBuilder {
    fn register_component<T>(&mut self) -> &mut Self
    where
        T: Properties + DeserializeProperty + Component + FromResources,
    {
        {
            let type_registry = self.app.resources.get::<TypeRegistry>().unwrap();
            type_registry.component.write().register::<T>();
            type_registry.property.write().register::<T>();
        }
        self
    }

    fn register_properties<T>(&mut self) -> &mut Self
    where
        T: Properties + DeserializeProperty + Component + FromResources,
    {
        {
            let type_registry = self.app.resources.get::<TypeRegistry>().unwrap();
            type_registry.property.write().register::<T>();
        }
        self
    }

    fn register_property<T>(&mut self) -> &mut Self
    where
        T: Property + DeserializeProperty,
    {
        {
            let type_registry = self.app.resources.get::<TypeRegistry>().unwrap();
            type_registry.property.write().register::<T>();
        }
        self
    }
}
