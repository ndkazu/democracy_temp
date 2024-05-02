import React from 'react';
import { AppProvider } from './contexts/AppContext';
import { AccountProvider } from './contexts/AccountContext';
import { Layout } from './components/shared/Layout';
import { CouncilSessionProvider } from './contexts/CouncilSessionContext';
import { TaskProvider } from './contexts/TaskContext';

function App() {
  return (
    <AppProvider>
      <AccountProvider>
        <CouncilSessionProvider>
          <TaskProvider>
            <Layout />
          </TaskProvider>
        </CouncilSessionProvider>
      </AccountProvider>
    </AppProvider>
  );
}

export default App;
