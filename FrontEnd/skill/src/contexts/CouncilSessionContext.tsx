import { createContext, useContext, useReducer, ReactNode } from 'react';
import { CouncilSessionState, Proposal, DataType } from './types';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

const initialState: CouncilSessionState = {
  approved: false,
  selected_proposal: undefined,
  proposals: [],
  session_subject: '',
  session_closed: false,
  ayes: 0,
  nay: 0,
  council_members: [],
  datas: [],
};

type Action =
  | { type: 'SET_APPROVAL'; payload: boolean }
  | { type: 'SET_SELECTED_PROPOSAL'; payload: Proposal }
  | { type: 'SET_PROPOSALS'; payload: Proposal[] }
  | { type: 'SET_SESSION_SUBJECT'; payload: string }
  | { type: 'SET_SESSION_CLOSE'; payload: boolean }
  | { type: 'SET_AYES'; payload: number }
  | { type: 'SET_NAY'; payload: number }
  | { type: 'SET_COUNCIL_MEMBERS'; payload: InjectedAccountWithMeta[] }
  | { type: 'SET_DATAS'; payload: DataType[] };

function reducer(state: CouncilSessionState, action: Action): CouncilSessionState {
  switch (action.type) {
    case 'SET_APPROVAL':
      return { ...state, approved: action.payload };
    case 'SET_SELECTED_PROPOSAL':
      return { ...state, selected_proposal: action.payload };
    case 'SET_PROPOSALS':
      return { ...state, proposals: action.payload };
    case 'SET_SESSION_CLOSE':
      return { ...state, session_closed: action.payload };
    case 'SET_SESSION_SUBJECT':
      return { ...state, session_subject: action.payload };
    case 'SET_AYES':
      return { ...state, ayes: action.payload };
    case 'SET_NAY':
      return { ...state, nay: action.payload };
    case 'SET_COUNCIL_MEMBERS':
      return { ...state, council_members: action.payload };
    case 'SET_DATAS':
      return { ...state, datas: action.payload };
    default:
      return state;
  }
}

type CouncilSessionContextType = CouncilSessionState & {
  dispatch2: React.Dispatch<Action>;
};
const CouncilSessionContext = createContext<CouncilSessionContextType>({
  ...initialState,
  dispatch2: () => {},
});

type Props = {
  children: ReactNode;
};

export function CouncilSessionProvider({ children }: Props) {
  const [
    {
      approved,
      selected_proposal,
      proposals,
      session_subject,
      session_closed,
      ayes,
      nay,
      council_members,
      datas,
    },
    dispatch2,
  ] = useReducer(reducer, initialState);
  return (
    <CouncilSessionContext.Provider
      value={{
        approved,
        selected_proposal,
        proposals,
        session_subject,
        session_closed,
        ayes,
        nay,
        council_members,
        datas,
        dispatch2,
      }}
    >
      {children}
    </CouncilSessionContext.Provider>
  );
}

export const useConcilSessionContext = () => useContext(CouncilSessionContext);
