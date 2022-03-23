import type { Principal } from '@dfinity/principal';
export type Profile = Profile_2;
export interface Profile_2 {
  'name' : string,
  'description' : string,
  'keywords' : Array<string>,
}
export type RawRand = [Array<number>];
export interface _SERVICE {
  'balance' : () => Promise<bigint>,
  'balance128' : () => Promise<bigint>,
  'get' : (arg_0: string) => Promise<Profile_2>,
  'getSelf' : () => Promise<Profile_2>,
  'greet' : (arg_0: string) => Promise<string>,
  'increment' : () => Promise<undefined>,
  'm_caller' : () => Promise<string>,
  'm_data_certificate' : () => Promise<string>,
  'm_id' : () => Promise<string>,
  'm_stable64_grow' : () => Promise<undefined>,
  'm_stable_size' : () => Promise<bigint>,
  'm_time' : () => Promise<bigint>,
  'path_test' : () => Promise<string>,
  'raw_rand' : () => Promise<RawRand>,
  'search' : (arg_0: string) => Promise<[] | [Profile_2]>,
  'set' : (arg_0: bigint) => Promise<undefined>,
  'update' : (arg_0: Profile_2) => Promise<undefined>,
}
