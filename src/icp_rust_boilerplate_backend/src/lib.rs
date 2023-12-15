#[macro_use]
extern crate serde;
use candid::Principal;
use ic_cdk::caller;
//use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod types;
use types::*;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ADMIN: RefCell<AdminCell> = RefCell::new(
        AdminCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), AdminPrincipal("".to_string()))
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


#[ic_cdk::init]
fn init() {
    let _ = ADMIN.with(|service| service.borrow_mut().set(AdminPrincipal(caller().to_string())));
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

fn is_caller_admin() -> Result<(), Error> {
    let admin = ADMIN.with(|service| {
        let current_value = service.borrow().get().clone();
        current_value
    });
    let convert_to_admin_principal = Principal::from_text(admin.0.clone());
    if convert_to_admin_principal.is_err(){
        return Err(Error::Unauthorized { msg: format!("Couldn't verify whether caller was the admin") })
    }
    let admin_principal = convert_to_admin_principal.unwrap();

    if admin_principal != caller(){
        return Err(Error::Unauthorized { msg: format!("Not admin of the canister") })
    }else{
        Ok(())
    }
}

#[ic_cdk::update]
fn add_port(port_payload: PortPayload) -> Result<Port, Error> {
    is_caller_admin()?;
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
    Ok(port)
}

#[ic_cdk::update]
fn add_user(user_payload: UserPayload) -> Result<User, Error> {
    if user_payload.email.trim().is_empty() || user_payload.username.trim().is_empty(){
        return Err(Error::InvalidUserPayload{msg: format!("Payload cannot contain empty values"), payload: user_payload})
    }
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
    Ok(user)
}

#[ic_cdk::update]
fn update_port(id: u64, payload: PortPayload) -> Result<Port, Error> {
    is_caller_admin()?;
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
    if payload.email.trim().is_empty() || payload.username.trim().is_empty(){
        return Err(Error::InvalidUserPayload{msg: format!("Payload cannot contain empty values"), payload})
    }
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
    is_caller_admin()?;
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
    is_caller_admin()?;
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
    is_caller_admin()?;
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
fn get_admin() -> AdminPrincipal {
    ADMIN.with(|service| {
        service.borrow().get().clone()
    })
}

#[ic_cdk::update]
fn transfer_ships_admin(source_port_id: u64, destination_port_id: u64, num_ships: u32) -> Result<(), Error> {
    // Check if the caller is an admin
    is_caller_admin()?;
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
    InvalidUserPayload{msg: String, payload: UserPayload},
    InvalidPortPayload{msg: String, payload: PortPayload}
}

// need this to generate candid
ic_cdk::export_candid!();
