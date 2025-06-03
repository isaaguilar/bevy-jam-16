use bevy::{
    asset::{Asset, Handle},
    ecs::{resource::Resource, system::IntoObserverSystem},
    prelude::{Bundle, Commands, Component, Event, OnAdd, Trigger},
};

/// Add the provided function to all entities with component T.
/// Usage: app.add_observer(add_observer_to_component::<SomeComponent, _, _, _, _>(
///           observer_function_to_be_added_to_all_entities_with_that_component,
///      ));

pub fn add_observer_to_component<T, S, E, B, M>(
    observer_function: S,
) -> impl FnMut(Trigger<OnAdd, T>, Commands) -> ()
where
    T: Component,
    B: Bundle,
    E: Event + 'static,
    S: IntoObserverSystem<E, B, M> + Send + Sync + Clone,
{
    move |trigger: Trigger<OnAdd, T>, mut commands: Commands| {
        commands
            .entity(trigger.target())
            .observe(observer_function.clone());
    }
}

pub trait GimmieFn<T, U>: 'static + Sync + Send + Fn(&U) -> Handle<T>
where
    T: Asset,
    U: Resource,
{
}

impl<F, T, U> GimmieFn<T, U> for F
where
    F: Fn(&U) -> Handle<T> + Send + Sync + 'static,
    T: Asset,
    U: Resource,
{
}
