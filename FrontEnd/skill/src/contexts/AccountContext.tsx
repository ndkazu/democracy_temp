import { createContext, useContext, useReducer, ReactNode } from 'react';
import { AccountState } from './types';
import BN from 'bn.js';

const initialState: AccountState = {
  address: '',
  user_name: '',
  ver_skills: [],
  unver_skills: [],
  balance: undefined,
  user_sp: 0,
  user_xp: 0,
  user_wage: 0,
};

type Action =
  | { type: 'SET_ADDRESS'; payload: string }
  | { type: 'SET_USER_NAME'; payload: string }
  | { type: 'SET_VER_SKILLS'; payload: string[] }
  | { type: 'SET_UNVER_SKILLS'; payload: string[] }
  | { type: 'SET_BALANCE'; payload: BN }
  | { type: 'SET_SP'; payload: number }
  | { type: 'SET_XP'; payload: number }
  | { type: 'SET_wage'; payload: number };

function reducer(state: AccountState, action: Action): AccountState {
  switch (action.type) {
    case 'SET_ADDRESS':
      return { ...state, address: action.payload };

    case 'SET_USER_NAME':
      return { ...state, user_name: action.payload };

    case 'SET_VER_SKILLS':
      return { ...state, ver_skills: action.payload };

    case 'SET_UNVER_SKILLS':
      return { ...state, unver_skills: action.payload };

    case 'SET_BALANCE':
      return { ...state, balance: action.payload };

    case 'SET_SP':
      return { ...state, user_sp: action.payload };

    case 'SET_XP':
      return { ...state, user_xp: action.payload };

    case 'SET_wage':
      return { ...state, user_wage: action.payload };

    default:
      return state;
  }
}

type AccountContextType = AccountState & {
  dispatch1: React.Dispatch<Action>;
};
const AccountContext = createContext<AccountContextType>({
  ...initialState,
  dispatch1: () => {},
});

type Props = {
  children: ReactNode;
};

export function AccountProvider({ children }: Props) {
  const [
    { address, user_name, ver_skills, unver_skills, balance, user_sp, user_xp, user_wage },
    dispatch1,
  ] = useReducer(reducer, initialState);
  return (
    <AccountContext.Provider
      value={{
        address,
        user_name,
        ver_skills,
        unver_skills,
        balance,
        user_sp,
        user_xp,
        user_wage,
        dispatch1,
      }}
    >
      {children}
    </AccountContext.Provider>
  );
}
export const useAccountContext = () => useContext(AccountContext);