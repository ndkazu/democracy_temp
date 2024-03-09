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
  total_employees_number: number;
  skills: string[];
}

export interface AccountState {
  address: Address;
  user_name: string;
  ver_skills: string[];
  unver_skills: string[];
  balance: BN | undefined;
  user_sp: number;
  user_xp: number;
  user_wage: number;
}

export interface CouncilSessionState {
  approved: boolean;
  session_subject: string;
  session_closed: boolean;
  ayes: number;
  nay: number;
  council_members: InjectedAccountWithMeta[];
}
