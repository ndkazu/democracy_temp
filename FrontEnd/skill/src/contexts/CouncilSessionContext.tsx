import { createContext, useContext, useReducer, ReactNode } from 'react';
import { CouncilSessionState } from './types';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

const initialState: CouncilSessionState = {
  approved: false,
  session_subject: '',
  session_closed: false,
  ayes: 0,
  nay: 0,
  council_members: [],
};

type Action =
  | { type: 'SET_APPROVAL'; payload: boolean }
  | { type: 'SET__SESSION_SUBJECT'; payload: string }
  | { type: 'SET_SESSION_CLOSE'; payload: boolean }
  | { type: 'SET_AYES'; payload: number }
  | { type: 'SET_NAY'; payload: number }
  | { type: 'SET_COUNCIL_MEMBERS'; payload: InjectedAccountWithMeta[] };

function reducer(state: CouncilSessionState, action: Action): CouncilSessionState {
  switch (action.type) {
    case 'SET_APPROVAL':
      return { ...state, approved: action.payload };
    case 'SET_SESSION_CLOSE':
      return { ...state, session_closed: action.payload };
    case 'SET__SESSION_SUBJECT':
      return { ...state, session_subject: action.payload };
    case 'SET_AYES':
      return { ...state, ayes: action.payload };
    case 'SET_NAY':
      return { ...state, nay: action.payload };
    case 'SET_COUNCIL_MEMBERS':
      return { ...state, council_members: action.payload };
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
  const [{ approved, session_subject, session_closed, ayes, nay, council_members }, dispatch2] =
    useReducer(reducer, initialState);
  return (
    <CouncilSessionContext.Provider
      value={{
        approved,
        session_subject,
        session_closed,
        ayes,
        nay,
        council_members,
        dispatch2,
      }}
    >
      {children}
    </CouncilSessionContext.Provider>
  );
}

export const useConcilSessionContext = () => useContext(CouncilSessionContext);
