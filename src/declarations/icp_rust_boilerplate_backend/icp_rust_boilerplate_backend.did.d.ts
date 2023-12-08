import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'NotFound' : { 'msg' : string } } |
  { 'Unauthorized' : { 'msg' : string } };
export interface Port {
  'id' : bigint,
  'current_ships' : number,
  'name' : string,
  'capacity' : number,
  'location' : string,
}
export interface PortPayload {
  'name' : string,
  'capacity' : number,
  'location' : string,
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Port } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : User } |
  { 'Err' : Error };
export interface User {
  'username' : string,
  'user_id' : bigint,
  'email' : string,
}
export interface UserPayload { 'username' : string, 'email' : string }
export interface _SERVICE {
  'add_port' : ActorMethod<[PortPayload], [] | [Port]>,
  'add_ship_to_port' : ActorMethod<[bigint], Result>,
  'add_user' : ActorMethod<[UserPayload], [] | [User]>,
  'delete_port' : ActorMethod<[bigint], Result_1>,
  'delete_user' : ActorMethod<[bigint], Result_2>,
  'get_admin' : ActorMethod<[], bigint>,
  'get_all_ports' : ActorMethod<[], Array<Port>>,
  'get_all_users' : ActorMethod<[], Array<User>>,
  'get_port' : ActorMethod<[bigint], Result_1>,
  'get_user' : ActorMethod<[bigint], Result_2>,
  'ships_arrival' : ActorMethod<[bigint, number], Result>,
  'transfer_ships_admin' : ActorMethod<
    [bigint, bigint, number, bigint],
    Result
  >,
  'update_port' : ActorMethod<[bigint, PortPayload], Result_1>,
  'update_user' : ActorMethod<[bigint, UserPayload], Result_2>,
}
