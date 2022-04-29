import type { Principal } from '@dfinity/principal';
export interface Address { 'id' : number, 'name' : [] | [string] }
export type HeaderField = [string, string];
export interface HttpQuery {
  'uri' : string,
  'method' : string,
  'body' : Array<number>,
  'headers' : Array<HttpQueryHeaderField>,
}
export type HttpQueryHeaderField = [Array<number>, Array<number>];
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Array<number>,
  'headers' : Array<HeaderField>,
  'upgrade' : [] | [boolean],
}
export type Profile = Profile_2;
export interface Profile_2 {
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export type RawRand = [Array<number>];
export interface _SERVICE {
  'add_address' : (arg_0: Address) => Promise<undefined>,
  'all_address' : () => Promise<Array<Address>>,
  'balance' : () => Promise<bigint>,
  'balance128' : () => Promise<bigint>,
  'cat' : (arg_0: string) => Promise<string>,
  'clear_get_http_request_history' : () => Promise<undefined>,
  'create_file' : (arg_0: string, arg_1: string) => Promise<Array<string>>,
  'get' : (arg_0: string) => Promise<Profile_2>,
  'getSelf' : () => Promise<Profile_2>,
  'get_address' : (arg_0: number) => Promise<[] | [Address]>,
  'get_all_file' : () => Promise<Array<string>>,
  'get_current_time_list' : () => Promise<Array<bigint>>,
  'get_http_request_history' : () => Promise<Array<HttpQuery>>,
  'get_http_update_request_history' : () => Promise<Array<HttpRequest>>,
  'get_test' : () => Promise<Array<number>>,
  'greet' : (arg_0: string) => Promise<string>,
  'increment' : () => Promise<undefined>,
  'ls' : (arg_0: string) => Promise<Array<string>>,
  'm_caller' : () => Promise<string>,
  'm_data_certificate' : () => Promise<string>,
  'm_id' : () => Promise<string>,
  'm_stable64_grow' : () => Promise<undefined>,
  'm_stable_read' : () => Promise<Array<number>>,
  'm_stable_size' : () => Promise<bigint>,
  'm_time' : () => Promise<bigint>,
  'mkdir' : (arg_0: string) => Promise<undefined>,
  'path_test' : () => Promise<string>,
  'raw_rand' : () => Promise<Array<number>>,
  'rm' : (arg_0: string) => Promise<undefined>,
  'search' : (arg_0: string) => Promise<[] | [Profile_2]>,
  'set' : (arg_0: bigint) => Promise<undefined>,
  'test' : () => Promise<undefined>,
  'update' : (arg_0: Profile_2) => Promise<undefined>,
  'write_file' : (arg_0: string, arg_1: string) => Promise<undefined>,
}
