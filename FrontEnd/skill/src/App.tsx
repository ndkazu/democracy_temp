import React from 'react';
import { AppProvider } from './contexts/AppContext';
import { Layout } from './components/shared/Layout';

function App() {
  return (
    <AppProvider>
      <Layout />
    </AppProvider>
  );
}

export default App;