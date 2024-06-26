import { createContext, useContext, useReducer, ReactNode } from 'react';
import { ApiPromise } from '@polkadot/api';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { AppState } from './types';
import BN from 'bn.js';

const initialState: AppState = {
  api: null,
  accounts: [],
  selectedAccount: undefined,
  selectedAddress: '',
  blocks: '',
  treasury_balance: '',
  web3Name: undefined,
  total_employees_number: 0,
  skills: [],
};

type Action =
  | { type: 'SET_API'; payload: ApiPromise | null }
  | { type: 'SET_ACCOUNTS'; payload: InjectedAccountWithMeta[] }
  | { type: 'SET_SELECTED_ACCOUNT'; payload: InjectedAccountWithMeta | undefined }
  | { type: 'SET_SELECTED_ADDRESS'; payload: string }
  | { type: 'SET_BLOCKS'; payload: string }
  | { type: 'SET_TREASURY_BALANCE'; payload: string }
  | { type: 'SET_WEB3_NAME'; payload: string | undefined }
  | { type: 'SET_EMPLOYEES_NBR'; payload: number }
  | { type: 'SET_SKILLS'; payload: string[] };

function reducer(state: AppState, action: Action): AppState {
  switch (action.type) {
    case 'SET_API':
      return { ...state, api: action.payload };
    case 'SET_ACCOUNTS':
      return { ...state, accounts: action.payload };
    case 'SET_SELECTED_ACCOUNT':
      return { ...state, selectedAccount: action.payload };
    case 'SET_SELECTED_ADDRESS':
      return { ...state, selectedAddress: action.payload };
    case 'SET_BLOCKS':
      return { ...state, blocks: action.payload };
    case 'SET_TREASURY_BALANCE':
      return { ...state, treasury_balance: action.payload };
    case 'SET_WEB3_NAME':
      return { ...state, web3Name: action.payload };
    case 'SET_EMPLOYEES_NBR':
      return { ...state, total_employees_number: action.payload };
    case 'SET_SKILLS':
      return { ...state, skills: action.payload };

    default:
      return state;
  }
}

type AppContextType = AppState & {
  dispatch: React.Dispatch<Action>;
};
const AppContext = createContext<AppContextType>({
  ...initialState,
  dispatch: () => {},
});

type Props = {
  children: ReactNode;
};

export function AppProvider({ children }: Props) {
  const [
    {
      api,
      accounts,
      selectedAccount,
      selectedAddress,
      blocks,
      treasury_balance,
      web3Name,
      total_employees_number,
      skills,
    },
    dispatch,
  ] = useReducer(reducer, initialState);
  return (
    <AppContext.Provider
      value={{
        api,
        accounts,
        selectedAccount,
        selectedAddress,
        blocks,
        treasury_balance,
        web3Name,
        total_employees_number,
        skills,
        dispatch,
      }}
    >
      {children}
    </AppContext.Provider>
  );
}
export const useAppContext = () => useContext(AppContext);
