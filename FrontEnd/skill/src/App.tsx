import React from 'react';
import { AppProvider } from './contexts/AppContext';
import { AccountProvider } from './contexts/AccountContext';
import { Layout } from './components/shared/Layout';

function App() {
  return (
    <AppProvider>
      <AccountProvider>
        <Layout />
      </AccountProvider>
    </AppProvider>
  );
}

export default App;
