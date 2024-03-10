import React from 'react';
import { Button, Checkbox } from 'antd';
import type { CheckboxProps } from 'antd';
import { useState, FormEvent } from 'react';
import { SkillLevel } from '../../contexts/types';
import { SkillFamily } from '../../contexts/types';
import { useAccountContext } from '../../contexts/AccountContext';
import { useAppContext } from '../../contexts/AppContext';
import { web3FromAddress } from '@polkadot/extension-dapp';
type SkProp = {
  index: number;
  name: string;
  level: SkillLevel;
  type: SkillFamily;
};

export default function SkillForm() {
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

  const [sub, setSub] = useState(false);
  const [infos, setInfos] = useState<SkProp>({
    index: 0,
    name: '',
    level: SkillLevel.Level1,
    type: SkillFamily.Technical,
  });

  const onChange: CheckboxProps['onChange'] = (e) => {
    console.log(`checked = ${e.target.checked}`);
    setSub(!sub);
  };

  function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
  }

  const getSkill = async () => {
    if (!api || !selectedAccount) return;
    if (sub === false) {
      let who = selectedAccount.address;
      const injector = await web3FromAddress(who);
      const tx = api.tx.skillsModule.addMySkills(infos.index);
      tx.signAndSend(who, { signer: injector.signer }, ({ status }) => {
        if (status.isInBlock) {
          console.log(`Completed at block hash #${status.asInBlock.toString()}`);
        } else {
          console.log(`Current status: ${status.type}`);
        }
      }).catch((error) => {
        console.log(':( transaction failed', error);
      });
    } else {
      let who = selectedAccount.address;
      const injector = await web3FromAddress(who);
      const tx = await api.tx.skillsModule.submitSkill(infos.name, infos.type, infos.level);
      tx.signAndSend(who, { signer: injector.signer }, ({ status }) => {
        if (status.isInBlock) {
          console.log(`Completed at block hash #${status.asInBlock.toString()}`);
        } else {
          console.log(`Current status: ${status.type}`);
        }
      }).catch((error) => {
        console.log(':( transaction failed', error);
      });
    }
  };
  const fieldStyle = 'flex flex-col mb-2';
  return (
    <div className="flex flex-col py-10 max-w-md mx-auto">
      <h2 className="font-bold underline mb-3">Skill Info</h2>
      <Checkbox onChange={onChange}>SubmitSkill</Checkbox>

      <form onSubmit={handleSubmit}>
        {sub === false ? (
          <p>
            <div className={fieldStyle}>
              <label htmlFor="index">Skill Index</label>
              <input
                type="number"
                id="index"
                value={infos.index}
                onChange={(e) => setInfos({ ...infos, index: Number(e.target.value) })}
              />
            </div>
          </p>
        ) : null}

        {sub === true ? (
          <p>
            <div className={fieldStyle}>
              <label htmlFor="name">Skill Name</label>
              <input
                type="text"
                id="name"
                value={infos.name}
                onChange={(e) => setInfos({ ...infos, name: e.target.value })}
              />
            </div>

            <div className={fieldStyle}>
              <label htmlFor="level">Skill Level</label>
              <select
                id="level"
                value={infos.level}
                onChange={(e) =>
                  setInfos({
                    ...infos,
                    level: SkillLevel[e.target.value as keyof typeof SkillLevel],
                  })
                }
              >
                <option value=""></option>
                <option value={SkillLevel.Level1}>{SkillLevel.Level1}</option>
                <option value={SkillLevel.Level2}>{SkillLevel.Level2}</option>
                <option value={SkillLevel.Level3}>{SkillLevel.Level3}</option>
                <option value={SkillLevel.Level4}>{SkillLevel.Level4}</option>
              </select>
            </div>
            <div className={fieldStyle}>
              <label htmlFor="type">Skill Family</label>
              <select
                id="type"
                value={infos.type}
                onChange={(e) =>
                  setInfos({
                    ...infos,
                    type: SkillFamily[e.target.value as keyof typeof SkillFamily],
                  })
                }
              >
                <option value=""></option>
                <option value={SkillFamily.Soft}>{SkillFamily.Soft}</option>
                <option value={SkillFamily.Technical}>{SkillFamily.Technical}</option>
              </select>
            </div>
          </p>
        ) : null}

        <div>
          <Button onClick={getSkill}>Submit</Button>
        </div>
      </form>
    </div>
  );
}
