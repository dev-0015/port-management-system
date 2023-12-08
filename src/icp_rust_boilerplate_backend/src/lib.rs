#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
//use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Port {
    id: u64,
    name: String,
    location: String,
    capacity: u32,
    current_ships: u32,
}

impl Storable for Port {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Port {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    user_id: u64,
    username: String,
    email: String,
}

impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PORT_STORAGE: RefCell<StableBTreeMap<u64, Port, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct PortPayload {
    name: String,
    location: String,
    capacity: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UserPayload {
    username: String,
    email: String,
}

#[ic_cdk::query]
fn get_port(id: u64) -> Result<Port, Error> {
    match _get_port(&id) {
        Some(port) => Ok(port),
        None => Err(Error::NotFound {
            msg: format!("a port with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_user(user_id: u64) -> Result<User, Error> {
    match _get_user(&user_id) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("a user with id={} not found", user_id),
        }),
    }
}

#[ic_cdk::update]
fn add_port(port_payload: PortPayload) -> Option<Port> {
    let port_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment port id counter");

    let port = Port {
        id: port_id,
        name: port_payload.name,
        location: port_payload.location,
        capacity: port_payload.capacity,
        current_ships: 0,
    };

    do_insert_port(&port);
    Some(port)
}

#[ic_cdk::update]
fn add_user(user_payload: UserPayload) -> Option<User> {
    let user_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment user id counter");

    let user = User {
        user_id,
        username: user_payload.username,
        email: user_payload.email,
    };

    do_insert_user(&user);
    Some(user)
}

#[ic_cdk::update]
fn update_port(id: u64, payload: PortPayload) -> Result<Port, Error> {
    match PORT_STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(mut port) => {
            port.name = payload.name;
            port.location = payload.location;
            port.capacity = payload.capacity;
            do_insert_port(&port);
            Ok(port)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a port with id={}. port not found", id),
        }),
    }
}

// helper method to perform port insert.
fn do_insert_port(port: &Port) {
    PORT_STORAGE.with(|service| service.borrow_mut().insert(port.id, port.clone()));
}

#[ic_cdk::update]
fn update_user(user_id: u64, payload: UserPayload) -> Result<User, Error> {
    match USER_STORAGE.with(|service| service.borrow_mut().get(&user_id)) {
        Some(mut user) => {
            user.username = payload.username;
            user.email = payload.email;
            do_insert_user(&user);
            Ok(user)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a user with id={}. user not found", user_id),
        }),
    }
}

// helper method to perform user insert.
fn do_insert_user(user: &User) {
    USER_STORAGE.with(|service| service.borrow_mut().insert(user.user_id, user.clone()));
}

#[ic_cdk::update]
fn delete_port(id: u64) -> Result<Port, Error> {
    match PORT_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(port) => Ok(port),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a port with id={}. port not found.", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_user(user_id: u64) -> Result<User, Error> {
    match USER_STORAGE.with(|service| service.borrow_mut().remove(&user_id)) {
        Some(user) => Ok(user),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete a user with id={}. user not found.", user_id),
        }),
    }
}

// a helper method to get a port by id. used in get_port/update_port
fn _get_port(id: &u64) -> Option<Port> {
    PORT_STORAGE.with(|service| service.borrow().get(id))
}

// a helper method to get a user by id. used in get_user/update_user
fn _get_user(user_id: &u64) -> Option<User> {
    USER_STORAGE.with(|service| service.borrow().get(user_id))
}

#[ic_cdk::update]
fn add_ship_to_port(port_id: u64) -> Result<(), Error> {
    // Retrieve the port based on the given ID
    match _get_port(&port_id) {
        Some(mut port) => {
            // Increment the current_ships count
            port.current_ships += 1;

            // Update the port in storage
            do_insert_port(&port);

            Ok(())
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't add a ship to port with id={}. port not found", port_id),
        }),
    }
}

#[ic_cdk::update]
fn ships_arrival(port_id: u64, num_ships: u32) -> Result<(), Error> {
    // Retrieve the port based on the given ID
    match _get_port(&port_id) {
        Some(mut port) => {
            // Update the current_ships count based on the number of arriving ships
            port.current_ships += num_ships;

            // Update the port in storage
            do_insert_port(&port);

            Ok(())
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't handle ship arrival for port with id={}. port not found", port_id),
        }),
    }
}

#[ic_cdk::query]
fn get_all_ports() -> Vec<Port> {
    PORT_STORAGE.with(|service| service.borrow().iter().map(|(_, port)| port.clone()).collect())
}

#[ic_cdk::query]
fn get_all_users() -> Vec<User> {
    USER_STORAGE.with(|service| service.borrow().iter().map(|(_, user)| user.clone()).collect())
}

// Admin-related functionality
#[ic_cdk::query]
fn get_admin() -> u64 {
    // Return the hard-coded admin ID
    1
}

#[ic_cdk::update]
fn transfer_ships_admin(source_port_id: u64, destination_port_id: u64, num_ships: u32, admin_id: u64) -> Result<(), Error> {
    // Check if the caller is an admin
    if admin_id != get_admin() {
        return Err(Error::Unauthorized {
            msg: "only admin can transfer ships".to_string(),
        });
    }
    // Transfer ships
    let mut source_port = match _get_port(&source_port_id) {
        Some(port) => port,
        None => {
            return Err(Error::NotFound {
                msg: format!("source port with id={} not found", source_port_id),
            })
        }
    };

    // Check if there are enough ships in the source port
    if source_port.current_ships < num_ships {
        return Err(Error::NotFound {
            msg: format!("insufficient ships in source port with id={}", source_port_id),
        });
    }

    // Retrieve destination port
    let mut destination_port = match _get_port(&destination_port_id) {
        Some(port) => port,
        None => {
            return Err(Error::NotFound {
                msg: format!("destination port with id={} not found", destination_port_id),
            })
        }
    };

    // Transfer ships
    source_port.current_ships -= num_ships;
    destination_port.current_ships += num_ships;

    // Update ports in storage
    do_insert_port(&source_port);
    do_insert_port(&destination_port);

    Ok(())
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    Unauthorized { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
