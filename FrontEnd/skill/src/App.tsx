import React from 'react';
import './App.css';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';
import { ChangeEvent, useEffect } from 'react';
import { useAppContext } from './contexts/AppContext';
import Identicon from '@polkadot/react-identicon';

const NAME = 'skills-dapp';

function AccountModal() {
  const { api, accounts, selectedAccount, selectedAddress, blocks, dispatch } = useAppContext();

  const setup = async () => {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api0 = await ApiPromise.create({ provider: wsProvider });
    dispatch({ type: 'SET_API', payload: api0 });
  };

  const handleConnection = async () => {
    const extensions = await web3Enable(NAME);
    if (!extensions) {
      throw Error('NO_EXTENSION_FOUND');
    }
  };
}

function App() {
  return <div>"test"</div>;
}

export default App;
