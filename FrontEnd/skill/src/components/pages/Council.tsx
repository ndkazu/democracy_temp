import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { useEffect, MouseEvent, useState } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import { useConcilSessionContext } from '../../contexts/CouncilSessionContext';
import { DataType, Proposal } from '../../contexts/types';
import { web3FromAddress } from '@polkadot/extension-dapp';
import { NotificationTwoTone, WarningTwoTone } from '@ant-design/icons';
import { BN, formatBalance } from '@polkadot/util';
import { Card } from 'antd';
import { List } from 'antd';
import { Button } from 'antd';

export default function Council() {
  return <div>Council Page</div>;
}
