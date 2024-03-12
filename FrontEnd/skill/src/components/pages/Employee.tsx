import React, { useEffect } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import BN from 'bn.js';
import { toUnit } from '../shared/utils';
import { useAccountContext } from '../../contexts/AccountContext';
import { Card } from 'antd';
import SkillForm from '../shared/Skillform';

export default function Employee() {
  const { api, blocks, selectedAccount } = useAppContext();
  const {
    user_id,
    user_name,
    ver_skills,
    unver_skills,
    balance,
    user_sp,
    user_xp,
    user_wage,
    dispatch1,
  } = useAccountContext();

  useEffect(() => {
    if (!api || !selectedAccount) return;
    let address0 = selectedAccount.address;

    api.query.system.account(address0, ({ data: free }: { data: { free: BN } }) => {
      let { free: balance2 } = free;

      dispatch1({ type: 'SET_BALANCE', payload: balance2 });
      console.log('user balance', balance);
    });

    api.query.skillsModule.userUnverifiedSkills(address0, (data: any) => {
      let uver = data.toHuman();
      console.log('unverified:', uver[0]);
      dispatch1({ type: 'SET_UNVER_SKILLS', payload: uver });
      console.log('unverified:', unver_skills);
    });

    api.query.skillsModule.userVerifiedSkills(address0, (data: any) => {
      let ver = data.toHuman();
      console.log('Verified: ', ver);
      dispatch1({ type: 'SET_VER_SKILLS', payload: ver });
    });

    api.query.skillsModule.employeeLog(address0, (employee0: any) => {
      let emp = employee0.toHuman();
      emp
        ? dispatch1({ type: 'SET_USER_NAME', payload: emp.name })
        : dispatch1({ type: 'SET_USER_NAME', payload: 'John Doe' });
    });

    api.query.skillsModule.employeeLog(address0, (employee0: any) => {
      let emp = employee0.toHuman();

      emp ? dispatch1({ type: 'SET_SP', payload: emp.sp }) : console.log('no user');
      emp ? dispatch1({ type: 'SET_XP', payload: emp.xp }) : console.log('no user');
      emp ? dispatch1({ type: 'SET_WAGE', payload: emp.wage }) : console.log('no user');
      emp ? dispatch1({ type: 'SET_USER_ID', payload: emp.uid }) : console.log('no user');
    });
  }, [selectedAccount, dispatch1, blocks]);

  return (
    <div className="font-bold">
      <Card
        title={user_name === 'John Doe' ? 'Not an employee account!!!' : user_name}
        bordered={true}
        style={{ width: 400 }}
      >
        <p>User Id: {user_name === 'John Doe' ? '***' : user_id}</p>
        <p>
          User Token balance:{' '}
          {user_name === 'John Doe' || !balance ? '***' : toUnit(balance, 3).toString()}
        </p>
        <p>
          User wage: {user_name === 'John Doe' || !user_wage ? '***' : user_wage.toString()} USD
        </p>
        <p>User SP: {user_name === 'John Doe' ? '***' : user_sp}</p>
        <p>User XP:{user_name === 'John Doe' ? '***' : user_xp}</p>
        <p>
          User Verified skills:
          {user_name === 'John Doe' || ver_skills.length === 0
            ? 'No verified skills'
            : ver_skills.map((skn: any) => (
                <p className="font-light">
                  {skn.metadata}-{skn.skillLevel}
                </p>
              ))}
        </p>
        <p>
          User Unverified Skills:
          {user_name === 'John Doe' || unver_skills.length === 0
            ? `No unverified skill`
            : unver_skills.map((skn: any) => (
                <p className="font-light">
                  {skn.metadata}-{skn.skillLevel}
                </p>
              ))}
        </p>
      </Card>
      <SkillForm />
    </div>
  );
}
