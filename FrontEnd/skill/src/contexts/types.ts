import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { ApiPromise } from '@polkadot/api';

export type Address = string | undefined;
export interface AppState {
  api: ApiPromise | null;
  accounts: InjectedAccountWithMeta[];
  selectedAccount: InjectedAccountWithMeta | undefined;
  selectedAddress: Address;
  blocks: string;
  treasury_balance: string;
  web3Name: string | undefined;
  total_employees_number: number;
  skills: string[];
}

export interface AccountState {
  user_id: number;
  address: Address;
  user_name: string;
  ver_skills: string[];
  unver_skills: string[];
  balance: string;
  user_sp: number;
  user_xp: number;
  user_wage: string;
}

export interface CouncilSessionState {
  approved: boolean;
  selected_proposal: Proposal | undefined;
  proposals: Proposal[];
  session_subject: string;
  session_closed: boolean;
  ayes: number;
  nay: number;
  council_members: InjectedAccountWithMeta[];
  datas: DataType[];
}

export interface TaskState {
  task_owner: string;
  task_id: number;
  task_list: string[];
  task_description: string | undefined;
  active_curator: InjectedAccountWithMeta | undefined;
}

export interface Proposal {
  voter_id: InjectedAccountWithMeta | undefined;
  Referendum_account: InjectedAccountWithMeta | undefined;
  session_closed: boolean;
  approved: boolean;
  ayes: number;
  nay: number;
  hash: string;
  infos: string;
}

export interface DataType {
  name: string | undefined;
  status: string;
  referendum: string;
  hash: string;
  infos: string;
}

export enum SkillLevel {
  Level1 = 'Level1',
  Level2 = 'Level2',
  Level3 = 'Level3',
  Level4 = 'Level4',
}
export enum SkillFamily {
  Soft = 'Soft',
  Technical = 'Technical',
}
