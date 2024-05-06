import React, { useEffect, useState } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import { toUnit } from '../shared/utils';
import { useAccountContext } from '../../contexts/AccountContext';
import { useTaskContext } from '../../contexts/TaskContext';
import { Card, List, Space } from 'antd';
import SkillForm from '../shared/Skillform';
import TaskForm from '../shared/MarketForm';
import { BN, formatBalance } from '@polkadot/util';
import { TaskProp } from '../shared/MarketForm';

export type TaskData = {
  address: string;
  task_owner: string;
  task_id: number;
  reward: string;
  needed_skills: string[];
};
export default function Market() {
  const { api, blocks, accounts, selectedAccount } = useAppContext();
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
  const [infos, setInfos] = useState<TaskData[]>([]);
  const [tsk, setTsk] = useState<TaskData>({
    address: '',
    task_owner: '',
    task_id: 0,
    reward: '',
    needed_skills: [],
  });

  const getTasksInfos0 = async () => {
    if (!api || !selectedAccount) return;
    let infos_all: TaskData[] = [];

    await api.query.bounties.bounties.entries((bount: any[]) => {
      if (bount.length === 0) return;
      bount.forEach(([key, value]) => {
        let datas = value.toHuman();
        let key0 = key.toHuman();
        formatBalance.setDefaults({ decimals: 11, unit: 'USD' });
        let data0: string[] = datas.value.toString().split(',');
        let data = data0.join('');
        let reward = formatBalance(new BN(data), {
          withSi: true,
          withZero: false,
        });

        let dum = { ...tsk, address: datas.proposer, task_id: key0, reward: reward };

        infos_all.push(dum);
        setInfos(infos_all);
      });
    });
  };

  function getTasksInfos1() {
    if (!api || !selectedAccount) return;
    let infos_all: TaskData[] = [];
    infos.forEach((x) => {
      api.query.skillsModule.employeeLog(x.address, (y: any) => {
        let employee = y.toHuman();
        let name = employee.name;
        let ts = { ...x, task_owner: name };
        infos_all.push(ts);

        setInfos(infos_all);
      });
    });
  }

  useEffect(() => {
    if (!api || !selectedAccount) return;

    getTasksInfos0();
    getTasksInfos1();
    console.log(infos);
  }, [api, selectedAccount, blocks]);

  const style1 = { width: 310, height: 200, background: `white`, Space: 5 };

  return (
    <div>
      <h1 className=" text-center text-2xl font-bold">Task Market Page</h1>
      <h2 className=" text-center italic text-blue-500">For employees, by employees</h2>

      <div className=" flex flex-row justify-center p-6 space-x-20">
        <p className="flex flex-col">
          <p className=" font-bold italic text-xl underline">Create a Task</p>
          <TaskForm />
        </p>
        <p className="flex flex-col">
          <p className=" font-bold italic text-xl underline">Select a Task</p>
          <div
            id="scrollableDiv"
            style={{
              height: 600,
              overflow: 'auto',
              padding: '0 16px',
            }}
          >
            <List
              dataSource={infos}
              renderItem={(item) => (
                <Card style={style1}>
                  <List.Item key={item.address}>
                    <List.Item.Meta
                      title={<p>{item.address.slice(0, 6) + '...' + item.address.slice(-6, -1)}</p>}
                      description={
                        <div>
                          <p>Task Owner: {item.task_owner}</p>
                          <p>Task ID: {item.task_id}</p>
                          <p>Reward: {item.reward}</p>
                        </div>
                      }
                    />
                  </List.Item>
                </Card>
              )}
            ></List>
          </div>
        </p>
      </div>
    </div>
  );
}
