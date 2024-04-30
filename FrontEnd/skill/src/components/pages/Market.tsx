import React, { useEffect } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import BN from 'bn.js';
import { toUnit } from '../shared/utils';
import { useAccountContext } from '../../contexts/AccountContext';
import { useTaskContext } from '../../contexts/TaskContext';
import { Card } from 'antd';
import SkillForm from '../shared/Skillform';

export default function Market() {
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

  const { task_owner, task_id, task_list, task_description, active_curator, dispatch2 } =
    useTaskContext();

  useEffect(() => {
    if (!api || !selectedAccount) return;
    let address0 = selectedAccount.address;
    api.query.market.taskStat(address0, (data: string) => {
      console.log(data);
    });
  });

  return <div>Market Page</div>;
}
