export const idlFactory = ({ IDL }) => {
  const Address = IDL.Record({ 'id' : IDL.Nat32, 'name' : IDL.Opt(IDL.Text) });
  const Profile_2 = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'keywords' : IDL.Vec(IDL.Text),
  });
  const HttpQueryHeaderField = IDL.Tuple(IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8));
  const HttpQuery = IDL.Record({
    'uri' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HttpQueryHeaderField),
  });
  const HeaderField = IDL.Tuple(IDL.Text, IDL.Text);
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(HeaderField),
    'upgrade' : IDL.Opt(IDL.Bool),
  });
  return IDL.Service({
    'add_address' : IDL.Func([Address], [], []),
    'all_address' : IDL.Func([], [IDL.Vec(Address)], []),
    'balance' : IDL.Func([], [IDL.Nat], ['query']),
    'balance128' : IDL.Func([], [IDL.Nat], ['query']),
    'cat' : IDL.Func([IDL.Text], [IDL.Text], []),
    'clear_get_http_request_history' : IDL.Func([], [], []),
    'create_file' : IDL.Func([IDL.Text, IDL.Text], [IDL.Vec(IDL.Text)], []),
    'get' : IDL.Func([IDL.Text], [Profile_2], ['query']),
    'getSelf' : IDL.Func([], [Profile_2], ['query']),
    'get_address' : IDL.Func([IDL.Nat32], [IDL.Opt(Address)], []),
    'get_all_file' : IDL.Func([], [IDL.Vec(IDL.Text)], []),
    'get_current_time_list' : IDL.Func([], [IDL.Vec(IDL.Nat64)], []),
    'get_http_request_history' : IDL.Func([], [IDL.Vec(HttpQuery)], []),
    'get_http_update_request_history' : IDL.Func(
        [],
        [IDL.Vec(HttpRequest)],
        [],
      ),
    'get_test' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'increment' : IDL.Func([], [], []),
    'ls' : IDL.Func([IDL.Text], [IDL.Vec(IDL.Text)], []),
    'm_caller' : IDL.Func([], [IDL.Text], ['query']),
    'm_data_certificate' : IDL.Func([], [IDL.Text], ['query']),
    'm_id' : IDL.Func([], [IDL.Text], ['query']),
    'm_stable64_grow' : IDL.Func([], [], ['query']),
    'm_stable_read' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'm_stable_size' : IDL.Func([], [IDL.Nat], []),
    'm_time' : IDL.Func([], [IDL.Nat], ['query']),
    'mkdir' : IDL.Func([IDL.Text], [], []),
    'path_test' : IDL.Func([], [IDL.Text], []),
    'raw_rand' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'rm' : IDL.Func([IDL.Text], [], []),
    'search' : IDL.Func([IDL.Text], [IDL.Opt(Profile_2)], ['query']),
    'set' : IDL.Func([IDL.Nat], [], []),
    'test' : IDL.Func([], [], []),
    'update' : IDL.Func([Profile_2], [], []),
    'write_file' : IDL.Func([IDL.Text, IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
