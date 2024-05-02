import React from 'react';
import { AppProvider } from './contexts/AppContext';
import { AccountProvider } from './contexts/AccountContext';
import { Layout } from './components/shared/Layout';
import { CouncilSessionProvider } from './contexts/CouncilSessionContext';

function App() {
  return (
    <AppProvider>
      <AccountProvider>
        <CouncilSessionProvider>
          <Layout />
        </CouncilSessionProvider>
      </AccountProvider>
    </AppProvider>
  );
}

export default App;
