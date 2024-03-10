import React, { useEffect } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import BN from 'bn.js';
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
      let { free: balance1 } = free;

      dispatch({ type: 'SET_TREASURY_BALANCE', payload: balance1 });
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

  return (
    <div>
      <h1 className="text-3xl text-slate-700 font-bold">DASHBOARD</h1>
      <p className="text-xl font-bold">
        Treasury Fund: {!treasury_balance ? '0' : toUnit(treasury_balance, 3).toString()} USD
      </p>
      <p className="font-bold">Total Number of employees: {total_employees_number}</p>
      <p className="font-bold">
        Skills List (Use the skill number to reference a particular skill):
        <div>
          {skills.map((sk: any, index: number) => (
            <div className="font-light">
              {index}-{sk.metadata}
            </div>
          ))}
        </div>
      </p>
    </div>
  );
}
