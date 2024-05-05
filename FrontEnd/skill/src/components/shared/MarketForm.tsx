import React from 'react';
import { Button } from 'flowbite-react';
import type { CheckboxProps } from 'antd';
import { useEffect, ChangeEvent } from 'react';
import { useForm, FieldError } from 'react-hook-form';
import { SkillLevel } from '../../contexts/types';
import { SkillFamily } from '../../contexts/types';
import { useAccountContext } from '../../contexts/AccountContext';
import { useAppContext } from '../../contexts/AppContext';
import { web3FromAddress } from '@polkadot/extension-dapp';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { useTaskContext } from '../../contexts/TaskContext';

export type TaskProp = {
  needed_sk_id: number;
  reward: number;
  description: string;
  curator: string | undefined;
};

export default function TaskForm() {
  const { task_owner, task_id, task_list, task_description, active_curator, dispatch2 } =
    useTaskContext();
  const { api, blocks, accounts, selectedAccount, selectedAddress, dispatch } = useAppContext();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<TaskProp>();

  function getEditorStyle(fieldError: FieldError | undefined) {
    return fieldError ? 'border-red-500' : ``;
  }

  const onSubmit = async (task: TaskProp) => {
    if (!api || !selectedAccount || !task.curator) return;
    let who = selectedAccount.address;
    const injector = await web3FromAddress(who);
    let converted_reward = task.reward * 1e11;
    console.log(`The address is:${task.curator}`);
    const tx = api.tx.market.proposeTask(
      task.needed_sk_id,
      converted_reward,
      task.description,
      task.curator,
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
    <div className={fieldStyle}>
      <form noValidate onSubmit={handleSubmit(onSubmit)}>
        <div className={fieldStyle}>
          <label htmlFor="needed_sk_id">Needed Skill Index</label>
          <input
            type="number"
            id="needed_sk_id"
            className={getEditorStyle(errors.needed_sk_id)}
            {...register('needed_sk_id')}
          />
        </div>

        <div className={fieldStyle}>
          <label htmlFor="reward">Task Reward (USD)</label>
          <input
            type="number"
            defaultValue="6"
            id="reward"
            {...register('reward', { min: { value: 6, message: 'Amount below minimum' } })}
          />
        </div>

        <div className={fieldStyle}>
          <label htmlFor="description">Task Description</label>
          <input
            type="text"
            id="description"
            className={getEditorStyle(errors.description)}
            {...register('description', { required: 'You need to provide a task description' })}
          />
        </div>

        <div className={fieldStyle}>
          <label htmlFor="curator">Curator</label>
          {accounts.length > 0 ? (
            <select
              id="account"
              className="outline-neutral-800 rounded-md py-1"
              {...register('curator')}
            >
              <option value="" disabled selected hidden key="nothing">
                Select an account
              </option>

              {accounts.map((account) => (
                <option value={account.address} key={account.address}>
                  {account.meta.name}
                </option>
              ))}
            </select>
          ) : null}
        </div>
        <div>
          <Button type="submit" className="bg-blue-600 text-white font-bold   text-xl">
            Submit
          </Button>
        </div>
      </form>
    </div>
  );
}
