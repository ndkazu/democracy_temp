import { createContext, useContext, useReducer, ReactNode } from 'react';
import { TaskState } from './types';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

const initialState: TaskState = {
  task_owner: '',
  task_id: 0,
  task_list: [],
  task_description: undefined,
  active_curator: undefined,
};

type Action =
  | { type: 'SET_TASK_OWNER'; payload: string }
  | { type: 'SET_TASK_ID'; payload: number }
  | { type: 'SET_TASK_LIST'; payload: string[] }
  | { type: 'SET_TASK_DESCRIPTION'; payload: string | undefined }
  | { type: 'SET_ACTIVE_CURATOR'; payload: InjectedAccountWithMeta | undefined };

function reducer(state: TaskState, action: Action): TaskState {
  switch (action.type) {
    case 'SET_TASK_OWNER':
      return { ...state, task_owner: action.payload };
    case 'SET_TASK_ID':
      return { ...state, task_id: action.payload };
    case 'SET_TASK_LIST':
      return { ...state, task_list: action.payload };
    case 'SET_TASK_DESCRIPTION':
      return { ...state, task_description: action.payload };
    case 'SET_ACTIVE_CURATOR':
      return { ...state, active_curator: action.payload };

    default:
      return state;
  }
}

type TaskContextType = TaskState & {
  dispatch2: React.Dispatch<Action>;
};

const TaskContext = createContext<TaskContextType>({
  ...initialState,
  dispatch2: () => {},
});

type Props = {
  children: ReactNode;
};

export function TaskProvider({ children }: Props) {
  const [{ task_owner, task_id, task_list, task_description, active_curator }, dispatch2] =
    useReducer(reducer, initialState);
  return (
    <TaskContext.Provider
      value={{
        task_owner,
        task_id,
        task_list,
        task_description,
        active_curator,
        dispatch2,
      }}
    >
      {children}
    </TaskContext.Provider>
  );
}
export const useTaskContext = () => useContext(TaskContext);
