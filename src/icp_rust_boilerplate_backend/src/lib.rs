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
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InspectionDetails {
    port_id: u64,
    ship_id: u64,
    inspection_date: u64, // Could be a timestamp
    findings: String,
    inspector_notes: String,
}
impl Storable for InspectionDetails {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InspectionDetails {
    const MAX_SIZE: u32 = 2048; // Adjusted as needed
    const IS_FIXED_SIZE: bool = false;
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PortTrafficReport {
    port_id: u64,
    start_date: u64,
    end_date: u64,
    total_arrivals: u32,
    total_departures: u32,
    // Additional metrics can be added as needed
}
impl Storable for PortTrafficReport {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for PortTrafficReport {
    const MAX_SIZE: u32 = 1024; // Adjust size as needed
    const IS_FIXED_SIZE: bool = false;
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EmergencyDetails {
    port_id: u64,
    emergency_type: String,
    description: String,
    timestamp: u64, // Could be a Unix timestamp
    // Additional fields as needed
}
impl Storable for EmergencyDetails {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EmergencyDetails {
    const MAX_SIZE: u32 = 1024; // Adjust size as needed
    const IS_FIXED_SIZE: bool = false;
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MaintenanceDetails {
    port_id: u64,
    maintenance_id: u64,
    description: String,
    scheduled_time: u64,  // Unix timestamp
    duration_hours: u32,  // Duration of maintenance
}
impl Storable for MaintenanceDetails {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for MaintenanceDetails {
    const MAX_SIZE: u32 = 2048; // Adjust size based on expected data size
    const IS_FIXED_SIZE: bool = false;
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ResourceUpdate {
    port_id: u64,
    resource_id: u64,
    resource_type: String,
    available: bool,
    last_updated: u64,  // Unix timestamp
}
impl Storable for ResourceUpdate {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for ResourceUpdate {
    const MAX_SIZE: u32 = 2048; // Adjust size based on expected data size
    const IS_FIXED_SIZE: bool = false;
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct DocumentDetails {
    document_id: u64,
    ship_id: u64,
    port_id: u64,
    document_type: String,
    content: String, // Assuming the document content is a string; adjust as needed
    timestamp: u64, // Unix timestamp for when the document was logged
}
impl Storable for DocumentDetails {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for DocumentDetails {
    const MAX_SIZE: u32 = 2048; // Adjust size based on expected data size
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
    static INSPECTION_STORAGE: RefCell<StableBTreeMap<u64, InspectionDetails, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    static PORT_TRAFFIC_REPORTS: RefCell<StableBTreeMap<u64, PortTrafficReport, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
    static EMERGENCY_DETAILS_STORAGE: RefCell<StableBTreeMap<u64, EmergencyDetails, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
    static MAINTENANCE_STORAGE: RefCell<StableBTreeMap<u64, MaintenanceDetails, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));
    static RESOURCE_STORAGE: RefCell<StableBTreeMap<u64, ResourceUpdate, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
    static DOCUMENT_STORAGE: RefCell<StableBTreeMap<u64, DocumentDetails, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8)))
    ));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct PortPayload {
    name: String,
    location: String,
    capacity: u32,
}
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct DocumentLogPayload {
    ship_id: u64,
    port_id: u64,
    document_type: String,
    content: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UserPayload {
    username: String,
    email: String,
}
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct InspectionPayload {
    ship_id: u64,
    inspection_date: u64,
    findings: String,
    inspector_notes: String,
}
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct TrafficAnalysisPayload {
    port_id: u64,
    start_date: u64,
    end_date: u64,
}
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct EmergencyDispatchPayload {
    port_id: u64,
    emergency_type: String,
    description: String,
    // Additional fields as needed
}
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct MaintenanceLogPayload {
    port_id: u64,
    description: String,
    scheduled_time: u64,
    duration_hours: u32,
}
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ResourceUpdatePayload {
    port_id: u64,
    resource_id: u64,
    resource_type: String,
    available: bool,
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
fn check_port_exists(_port_id: u64) -> bool {
    // Example logic to check if a port exists
    // In a real implementation, this might query a database or a data structure
    // For demonstration, let's assume it returns true
    true
}
#[ic_cdk::update]
fn dispatch_emergency_info(payload: EmergencyDispatchPayload) -> Result<(), Error> {
    // Validate the existence of the port (assume a function like `check_port_exists` exists)
    if !check_port_exists(payload.port_id) {
        return Err(Error::NotFound {
            msg: format!("Port with id={} not found.", payload.port_id),
        });
    }

    // Create an instance of EmergencyDetails
    let emergency_detail = EmergencyDetails {
        port_id: payload.port_id,
        emergency_type: payload.emergency_type,
        description: payload.description,
        timestamp: ic_cdk::api::time(), // Current time as a Unix timestamp
    };

    // Store the emergency detail in stable memory
    EMERGENCY_DETAILS_STORAGE.with(|storage| {
        storage.borrow_mut().insert(emergency_detail.port_id, emergency_detail.clone())
    });

    // Simulate notifying emergency contacts (for MVP, we just log the action)
    ic_cdk::println!("Emergency at port {}: {}. Notifying emergency contacts.", emergency_detail.port_id, emergency_detail.description);
    // In a real implementation, here you would add logic to actually notify contacts

    Ok(())
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
fn log_ship_inspection(port_id: u64, payload: InspectionPayload) -> Result<(), Error> {
    // Validate port and ship existence here...

    let inspection = InspectionDetails {
        port_id,
        ship_id: payload.ship_id,
        inspection_date: payload.inspection_date,
        findings: payload.findings,
        inspector_notes: payload.inspector_notes,
    };

    INSPECTION_STORAGE.with(|storage| {
        storage.borrow_mut().insert(inspection.ship_id, inspection)
    });

    Ok(())
}
#[ic_cdk::query]
fn analyze_port_traffic(payload: TrafficAnalysisPayload) -> Result<PortTrafficReport, Error> {
    // Logic to validate port existence and fetch data

    // Initialize counters for arrivals and departures
    let total_arrivals = 0;
    let total_departures = 0;

    // Logic to aggregate data based on the date range and port_id
    // This may involve iterating over a dataset of ship movements

    // Construct and store the traffic report
    let report = PortTrafficReport {
        port_id: payload.port_id,
        start_date: payload.start_date,
        end_date: payload.end_date,
        total_arrivals,
        total_departures,
    };

    PORT_TRAFFIC_REPORTS.with(|reports| reports.borrow_mut().insert(report.port_id, report.clone()));

    Ok(report)
}
#[ic_cdk::update]
fn log_maintenance_activity(payload: MaintenanceLogPayload) -> Result<(), Error> {
    // Logic to validate port existence and check for scheduling conflicts

    let maintenance_detail = MaintenanceDetails {
        port_id: payload.port_id,
        maintenance_id: generate_maintenance_id(),
        description: payload.description,
        scheduled_time: payload.scheduled_time,
        duration_hours: payload.duration_hours,
    };

    // Store maintenance detail
    MAINTENANCE_STORAGE.with(|storage| {
        storage.borrow_mut().insert(maintenance_detail.maintenance_id, maintenance_detail)
    });

    Ok(())
}
fn generate_maintenance_id() -> u64 {
    ic_cdk::api::time() // Using the current Unix timestamp as a unique ID
}

#[ic_cdk::update]
fn update_port_resources(payload: ResourceUpdatePayload) -> Result<(), Error> {
    // Logic to validate port existence and resource availability

    let resource_update = ResourceUpdate {
        port_id: payload.port_id,
        resource_id: payload.resource_id,
        resource_type: payload.resource_type,
        available: payload.available,
        last_updated: ic_cdk::api::time(),
    };

    // Store resource update
    RESOURCE_STORAGE.with(|storage| {
        storage.borrow_mut().insert(resource_update.resource_id, resource_update)
    });

    Ok(())
}
#[ic_cdk::update]
fn log_customs_documents(payload: DocumentLogPayload) -> Result<(), Error> {
    // Generate a unique document ID (assuming a function like `generate_document_id` exists)
    let document_id = generate_document_id();

    let document_detail = DocumentDetails {
        document_id,
        ship_id: payload.ship_id,
        port_id: payload.port_id,
        document_type: payload.document_type,
        content: payload.content,
        timestamp: ic_cdk::api::time(), // Current time as Unix timestamp
    };

    // Store the document detail in stable memory
    DOCUMENT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(document_detail.document_id, document_detail.clone())
    });

    Ok(())
}
fn generate_document_id() -> u64 {
    ic_cdk::api::time() // Current Unix timestamp
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