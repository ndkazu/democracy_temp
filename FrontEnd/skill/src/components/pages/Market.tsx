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
  const getTasksInfos = async () => {
    if (!api || !selectedAccount) return;
    let infos_all: string[] = [];
    let infos0: TaskData = tsk;
    let inf: TaskData[] = [];

    await api.query.bounties.bounties.entries((bount: any[]) => {
      bount.forEach(([key, value]) => {
        let datas = value.toHuman();
        infos0.address = datas.proposer;
        let key0 = key.toHuman();
        infos0.task_id = key0;
        formatBalance.setDefaults({ decimals: 11, unit: 'USD' });
        let data0: string[] = datas.value.toString().split(',');
        let data = data0.join('');
        let reward = formatBalance(new BN(data), {
          withSi: true,
          withZero: false,
        });
        infos0.reward = reward;
        let status = datas.status;
        api.queryMulti(
          [
            [api.query.skillsModule.employeeLog, infos0.address],
            [api.query.market.taskSkills, key0],
          ],
          ([employee, data]) => {
            let data_e: any = employee.toHuman();
            let name = data_e.name;
            let info0 = `Task ID: ${key0};Task Owner: ${name};Task Reward:${reward};Task Status:${status}`;
            infos_all.push(info0);
            let d0: any = data.toHuman();
            infos0.needed_skills = d0;
            infos0.task_owner = name;

            setTsk(infos0);
            console.log(tsk);
            inf.push(tsk);
            setInfos(inf);
          },
        );
      });

      dispatch2({ type: 'SET_TASK_LIST', payload: infos_all });
    });
  };

  useEffect(() => {
    if (!api || !selectedAccount) return;
    setInfos([]);
    getTasksInfos();
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
