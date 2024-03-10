import React from 'react';
import { Button, Checkbox } from 'antd';
import type { CheckboxProps } from 'antd';
import { useState, FormEvent } from 'react';
import { SkillLevel } from '../../contexts/types';
import { SkillFamily } from '../../contexts/types';
import { useAccountContext } from '../../contexts/AccountContext';
import { useAppContext } from '../../contexts/AppContext';
import { web3FromAddress } from '@polkadot/extension-dapp';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { useTaskContext } from '../../contexts/TaskContext';
import BN from 'bn.js';

type TaskProp = {
  needed_sk_id: number;
  reward: BN | undefined;
  description: string;
  curator: InjectedAccountWithMeta | undefined;
};

export default function TaskForm() {
  const { task_owner, task_id, task_list, task_description, active_curator, dispatch2 } =
    useTaskContext();
  const { api, blocks, accounts, selectedAccount, selectedAddress, dispatch } = useAppContext();
  const {
    user_id,
    address,
    user_name,
    ver_skills,
    unver_skills,
    balance,
    user_sp,
    user_xp,
    user_wage,
    dispatch1,
  } = useAccountContext();

  const [infos, setInfos] = useState<TaskProp>({
    needed_sk_id: 0,
    reward: undefined,
    description: '',
    curator: undefined,
  });
  function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
  }

  const submitTask = async () => {
    if (!api || !selectedAccount) return;
    let who = selectedAccount.address;
    const injector = await web3FromAddress(who);
    const tx = api.tx.marketModule.proposeTask(
      infos.needed_sk_id,
      infos.reward,
      infos.description,
      infos.curator,
    );
    tx.signAndSend(who, { signer: injector.signer }, ({ status }) => {
      if (status.isInBlock) {
        console.log(`Completed at block hash #${status.asInBlock.toString()}`);
      } else {
        console.log(`Current status: ${status.type}`);
      }
    }).catch((error) => {
      console.log(':( transaction failed', error);
    });
  };
  const fieldStyle = 'flex flex-col mb-2';
  return (
    <div>
      <h2>
        <form>
          <div>
            <label htmlFor="needed_sk_id">Needed Skill Index</label>
            <input
              type="number"
              id="needed_sk_id"
              value={infos.needed_sk_id}
              onChange={(e) => setInfos({ ...infos, needed_sk_id: Number(e.target.value) })}
            />
          </div>
          <div></div>
        </form>
      </h2>
    </div>
  );
}
