use specs::{Builder, Component, DispatcherBuilder, ReadStorage,
            System, VecStorage, World, WriteStorage, Read};

//Mark Position as a component type, some attribute/functionality an entity can have
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32
}

//Mark Position as a component type, some attribute/functionality an entity can have
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

//Create DeltaTime to use as a Resource (not a component, just a standalone bundle of data systems can read/write)
#[derive(Default)]
struct DeltaTime(f32);

//Create a struct for using as a system, can totally have internal data btw
struct HelloWorld;

//Implement system for HelloWorld
impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>; //We will read in all Entities with a Position Component

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() { //this is how we get an iterator for call Entitits with Position
            println!("Hello, {:?}", &position);
        }
    }
}

//Struct for second System, this will update Position based on velocity
struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (Read<'a, DeltaTime>, //Read in DeltaTime, this will need to be updated over time in the System
                       ReadStorage<'a, Velocity>, //Read in all entities with Velocity
                       WriteStorage<'a, Position>); //Write to all entities with Position, lets us have a mutable reference

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;

        // `Read` implements `Deref`, so it
        // coerces to `&DeltaTime`.
        let delta = delta.0;

        use specs::Join;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

pub fn specs_main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new() //dispatches all given systems in order
        .with(HelloWorld, "hello_world", &[]) //hello world will run as soon as dispatcher can run it
        .with(UpdatePos, "update_pos", &["hello_world"]) //update position will run after hello world has run
        .with(HelloWorld, "hello_updated", &["update_pos"]) //hello_updated will run after update_pos has run
        //.with_thread_local(RenderSys); this is here to remind how to render components properly
        .build();

    dispatcher.setup(&mut world.res); //register all Components, setup any Resources with Default implementations

    // Only the second entity will get a position update,
    // because the first one does not have a velocity.
    world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    

    dispatcher.dispatch(&mut world.res); //dispatch all systems
    world.maintain(); //update entity registry with any changes that occured in the Systems
}
