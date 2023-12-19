use candid::{Decode, Encode};
//use ic_cdk::api::time;
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, Storable};
use std::borrow::Cow;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;
pub type AdminCell = Cell<AdminPrincipal, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub struct AdminPrincipal(pub String);

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Port {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub capacity: u32,
    pub current_ships: u32,
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
impl Storable for AdminPrincipal {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for AdminPrincipal {
    const MAX_SIZE: u32 = 63;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub email: String,
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


#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct PortPayload {
    pub name: String,
    pub location: String,
    pub capacity: u32,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct UserPayload {
    pub username: String,
    pub email: String,
}
