import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { ApiPromise } from '@polkadot/api';
import BN from 'bn.js';

export type Address = string | undefined;
export interface AppState {
  api: ApiPromise | null;
  accounts: InjectedAccountWithMeta[];
  selectedAccount: InjectedAccountWithMeta | undefined;
  selectedAddress: Address;
  blocks: string;
  treasury_balance: BN | undefined;
  web3Name: string | undefined;
  total_skills_number: number;
}

export interface AccountContextState {
  address: Address;
  user_name: string;
  //ver_skills: Skill[];
  //unver_skills: Skill[];
  balance: BN | undefined;
  user_ver_skills_nbr: number;
  user_unv_skills_nbr: number;
  user_sp: number;
  user_xp: number;
  user_wage: number;
}
