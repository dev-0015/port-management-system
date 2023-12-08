export const idlFactory = ({ IDL }) => {
  const PortPayload = IDL.Record({
    'name' : IDL.Text,
    'capacity' : IDL.Nat32,
    'location' : IDL.Text,
  });
  const Port = IDL.Record({
    'id' : IDL.Nat64,
    'current_ships' : IDL.Nat32,
    'name' : IDL.Text,
    'capacity' : IDL.Nat32,
    'location' : IDL.Text,
  });
  const Error = IDL.Variant({
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'Unauthorized' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const UserPayload = IDL.Record({ 'username' : IDL.Text, 'email' : IDL.Text });
  const User = IDL.Record({
    'username' : IDL.Text,
    'user_id' : IDL.Nat64,
    'email' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : Port, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  return IDL.Service({
    'add_port' : IDL.Func([PortPayload], [IDL.Opt(Port)], []),
    'add_ship_to_port' : IDL.Func([IDL.Nat64], [Result], []),
    'add_user' : IDL.Func([UserPayload], [IDL.Opt(User)], []),
    'delete_port' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_user' : IDL.Func([IDL.Nat64], [Result_2], []),
    'get_admin' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_all_ports' : IDL.Func([], [IDL.Vec(Port)], ['query']),
    'get_all_users' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'get_port' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_user' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'ships_arrival' : IDL.Func([IDL.Nat64, IDL.Nat32], [Result], []),
    'transfer_ships_admin' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Nat32, IDL.Nat64],
        [Result],
        [],
      ),
    'update_port' : IDL.Func([IDL.Nat64, PortPayload], [Result_1], []),
    'update_user' : IDL.Func([IDL.Nat64, UserPayload], [Result_2], []),
  });
};
export const init = ({ IDL }) => { return []; };
