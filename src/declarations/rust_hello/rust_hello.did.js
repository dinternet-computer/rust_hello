export const idlFactory = ({ IDL }) => {
  const Address = IDL.Record({ 'id' : IDL.Nat32, 'name' : IDL.Opt(IDL.Text) });
  const Profile_2 = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'keywords' : IDL.Vec(IDL.Text),
  });
  return IDL.Service({
    'add_address' : IDL.Func([Address], [], []),
    'all_address' : IDL.Func([], [IDL.Vec(Address)], []),
    'balance' : IDL.Func([], [IDL.Nat], ['query']),
    'balance128' : IDL.Func([], [IDL.Nat], ['query']),
    'create_file' : IDL.Func([IDL.Text, IDL.Text], [IDL.Vec(IDL.Text)], []),
    'get' : IDL.Func([IDL.Text], [Profile_2], ['query']),
    'getSelf' : IDL.Func([], [Profile_2], ['query']),
    'get_address' : IDL.Func([IDL.Nat32], [Address], []),
    'get_all_file' : IDL.Func([], [IDL.Vec(IDL.Text)], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'increment' : IDL.Func([], [], []),
    'm_caller' : IDL.Func([], [IDL.Text], ['query']),
    'm_data_certificate' : IDL.Func([], [IDL.Text], ['query']),
    'm_id' : IDL.Func([], [IDL.Text], ['query']),
    'm_stable64_grow' : IDL.Func([], [], ['query']),
    'm_stable_read' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'm_stable_size' : IDL.Func([], [IDL.Nat], []),
    'm_time' : IDL.Func([], [IDL.Nat], ['query']),
    'path_test' : IDL.Func([], [IDL.Text], []),
    'raw_rand' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'search' : IDL.Func([IDL.Text], [IDL.Opt(Profile_2)], ['query']),
    'set' : IDL.Func([IDL.Nat], [], []),
    'update' : IDL.Func([Profile_2], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
