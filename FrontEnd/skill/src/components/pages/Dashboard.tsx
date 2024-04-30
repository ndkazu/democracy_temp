import React, { useEffect } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import { BN, formatBalance } from '@polkadot/util';
import { Card } from 'antd';
import { NavLink } from 'react-router-dom';
import { toUnit } from '../shared/utils';
const treasury_address = '5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z';

export default function Dashboard() {
  const {
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
  } = useAppContext();
  useEffect(() => {
    if (!api) return;

    api.query.system.account(treasury_address, ({ data: free }: { data: { free: BN } }) => {
      formatBalance.setDefaults({ decimals: 11, unit: 'USD' });
      const free0 = formatBalance(free.free, { withSi: true, withZero: false });

      dispatch({ type: 'SET_TREASURY_BALANCE', payload: free0 });
      console.log('Treasury balance:', treasury_balance);
    });

    api.query.skillsModule.employeesNumber((data: number) => {
      let data1 = Number(data.toString());
      dispatch({ type: 'SET_EMPLOYEES_NBR', payload: data1 });
    });

    api.query.skillsModule.skills((data: any) => {
      let data0 = data.toHuman();
      dispatch({ type: 'SET_SKILLS', payload: data0 });
      console.log(data0[0].metadata);
    });
  }, [blocks, api, dispatch]);

  const style1 = { width: 310, height: 250, background: `white` };
  return (
    <div>
      <h1 className="text-3xl text-slate-700 font-bold">DASHBOARD</h1>
      <p className="text-xl font-bold">
        Treasury Fund: {!treasury_balance ? '0' : treasury_balance}
      </p>
      <p className="font-bold">Total Number of employees: {total_employees_number}</p>
      <p className="font-bold">
        Skills List (Use the skill number to reference a particular skill):
        <div>
          <Card style={style1}>
            {skills.map((sk: any, index: number) => (
              <div className="font-light">
                {index}-{sk.metadata}
              </div>
            ))}
          </Card>
        </div>
      </p>
    </div>
  );
}
