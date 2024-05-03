import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { useEffect, MouseEvent, useState } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import { useConcilSessionContext } from '../../contexts/CouncilSessionContext';
import { DataType, Proposal } from '../../contexts/types';
import { web3FromAddress } from '@polkadot/extension-dapp';
import { Toast } from 'flowbite-react';
import { NotificationTwoTone, WarningTwoTone } from '@ant-design/icons';
import { BN, formatBalance } from '@polkadot/util';
import { Card } from 'antd';
import { List } from 'antd';
import { Button } from 'antd';

export default function Council() {
  const { api, accounts, selectedAccount, blocks } = useAppContext();
  const {
    approved,
    selected_proposal,
    proposals,
    session_subject,
    session_closed,
    ayes,
    nay,
    council_members,
    datas,
    dispatch3,
  } = useConcilSessionContext();

  const initprop: Proposal = {
    voter_id: undefined,
    Referendum_account: undefined,
    session_closed: false,
    approved: false,
    ayes: 0,
    nay: 0,
    hash: '',
    infos: '',
  };
  const [out, setOut] = useState<string[]>();
  const [voted, setVoted] = useState(false);
  const [event, setEvents] = useState('No Vote');
  const [showToast, setShowToast] = useState(false);
  const [warning, setWarning] = useState(false);
  const [treshold, setTres] = useState(0);
  const [close, setClose] = useState(true);
  const [tcouncil, setTcouncil] = useState(false);
  const [curator, setCurator] = useState(false);

  const getproposal = (item: MouseEvent) => {
    let txt = item.currentTarget.textContent;

    proposals.forEach((prop) => {
      let acc1 = prop.Referendum_account;
      if (!acc1) return;
      let name = acc1.address;
      if (!name || !txt) return;
      if (txt.includes(name.slice(0, 6) + '...' + name.slice(-6, -1))) {
        dispatch3({ type: `SET_SELECTED_PROPOSAL`, payload: prop });
      }
    });
  };

  const handleClose = async () => {
    if (!api || !selectedAccount || !selected_proposal) return;
    let who = selectedAccount.address;
    let prop = selected_proposal.Referendum_account?.address;
    if (!prop) return;
    let tx = await api.tx.skillsModule.councilClose(prop);
    if (tcouncil) {
      tx = await api.tx.market.councilClose(prop, curator);
    }
    const injector = await web3FromAddress(who);
    tx.signAndSend(who, { signer: injector.signer });
    setClose(true);
  };

  const handleClick = async (vote: boolean) => {
    if (!api || !selectedAccount || !selected_proposal) return;
    let who = selectedAccount.address;
    let prop = selected_proposal.Referendum_account?.address;
    if (!prop) return;

    let tx = await api.tx.skillsModule.councilVote(prop, vote);
    if (tcouncil) {
      tx = await api.tx.market.councilVote(prop, vote, curator);
    }
    const fees = await tx.paymentInfo(who);
    const injector = await web3FromAddress(who);
    tx.signAndSend(who, { signer: injector.signer }, ({ status, events, dispatchError }) => {
      if (dispatchError && status.isInBlock) {
        if (dispatchError.isModule) {
          console.log(`Current status: ${status.type}`);
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { docs, name, section } = decoded;
          setEvents(name.toString());
          setShowToast(true);
          setWarning(true);
        }
      } else if (status.isInBlock) {
        events.forEach(({ event: { method, section, data } }) => {
          let id = '';
          tcouncil ? (id = 'market') : (id = 'skillsModule');
          if (section.toString().includes(id)) {
            let meth = method.toString() + '\n';
            formatBalance.setDefaults({ decimals: 11, unit: 'USD' });
            let payed = formatBalance(new BN(fees.partialFee.toString()), {
              withSi: true,
              withZero: false,
            });
            setEvents(`${meth} =>Paid fees: ${payed} `);
            setShowToast(true);
            setWarning(false);
          }
        });
      } else {
        console.log(`Current status: ${status.type}`);
      }
    });
    setVoted(true);
  };

  function checkVote() {
    if (!api || !selectedAccount) return;
    api.query.council.voting.entries((all: any[]) => {
      all.forEach(([key, value]) => {
        api.query.skills.skillsProposalList(key, (prop: any) => {
          if (!prop) return;
          let inf = value.toHuman();
          let yes: string[] = inf.ayes;
          let no: string[] = inf.nays;

          if (yes.includes(selectedAccount.address) || no.includes(selectedAccount.address)) {
            let tres = yes.length + no.length;
            setTres(tres);
            setVoted(true);
          } else {
            setVoted(false);
            setTres(0);
          }
        });
      });
    });
  }

  function getDatas() {
    if (!api || !selectedAccount) return;
    let tdata: DataType[] = [];
    let props: Proposal[] = [];

    api.query.skillsModule.skillsProposalList.entries((all: any[]) => {
      if (!council_members.includes(selectedAccount)) return;
      all.forEach(([key, value]) => {
        let Prop = value.toHuman();
        if (!Prop) return;
        let acc0 = Prop.account;
        let acc1 = accounts.find((account) => account.address === acc0);
        if (!acc1) return;

        let status = Prop.approved.toString();
        let referendum = Prop.sessionClosed.toString();
        let hash = Prop.proposalHash.toString();
        let skill = Prop.skill;
        let infos = `${skill.metadata}:${skill.skillType}:${skill.skillLevel}:${skill.confirmed}:${skill.skillNumber}`;
        let dtype: DataType = {
          name: acc1.address,
          status,
          referendum,
          hash,
          infos,
        };

        let prop0: Proposal = {
          voter_id: selectedAccount,
          Referendum_account: acc1,
          session_closed: referendum,
          approved: status,
          ayes,
          nay,
          hash,
          infos,
        };
        props.push(prop0);
        tdata.push(dtype);
      });

      dispatch3({ type: `SET_DATAS`, payload: tdata });
      dispatch3({ type: `SET_PROPOSALS`, payload: props });
    });
  }

  function arrangeText0() {
    if (!selected_proposal) return;
    datas.forEach((x) => {
      if (selected_proposal.Referendum_account?.address === x.name) {
        let alltxt = selected_proposal.infos.split(':');
        let address0 = x.name;
        api?.query.skillsModule.employeeLog(address0, (datas0: any) => {
          let data = datas0.toHuman();
          let output00 = `Employee ID: ${data.uid};Employee Name: ${data.name};Proposed Skill: ${alltxt[0]};Skill Family: ${alltxt[1]};Skill Level: ${alltxt[2]}`;
          let txt = output00.split(';');
          setOut(txt);
          console.log(txt);
        });
      }
    });
  }

  useEffect(() => {
    if (!api || !selectedAccount) return;

    api.query.council.members((who: any[]) => {
      let members: InjectedAccountWithMeta[] = [];
      who.forEach((x) => {
        let y = x.toHuman();
        accounts.forEach((ac) => {
          if (ac.address === y) {
            members.push(ac);
          }
        });
      });
      //console.log(members);
      dispatch3({ type: 'SET_COUNCIL_MEMBERS', payload: members });
    });

    //dispatch3({ type: `SET_DATAS`, payload: [] });
    //dispatch3({ type: `SET_PROPOSALS`, payload: [] });
    getDatas();
    let val = '';
    if (!selected_proposal) {
      setOut(['']);
    } else {
      console.log('Selected!!');
      arrangeText0();
    }

    checkVote();
    if (datas.length === 0) {
      dispatch3({ type: 'SET_SELECTED_PROPOSAL', payload: initprop });
      setVoted(false);
    }
    if (treshold < 2) {
      setClose(true);
    } else {
      setClose(false);
    }
  }, [api, selectedAccount, blocks, dispatch3]);

  const style1 = { width: 310, height: 250, background: `white` };
  const style2 = { width: 310, height: 250, background: `#ffccc7` };
  const style3 = { width: 310, height: 250, background: `#f4ffb8` };
  const style4 = { width: 410, height: 400, background: `white` };
  if (!selectedAccount || !council_members.includes(selectedAccount)) {
    return (
      <div className=" flex font-bold text-5xl justify-center py-12">
        You Are not a Council Member
      </div>
    );
  } else {
    return (
      <div className="flex flex-row justify-between p-6">
        <div
          id="scrollableDiv"
          style={{
            height: 600,
            overflow: 'auto',
            padding: '0 16px',
            // border: '1px solid rgba(140, 140, 140, 0.35)',
          }}
        >
          <List
            dataSource={datas}
            renderItem={(item) => (
              <Card
                onClick={getproposal}
                hoverable
                cover={
                  <img
                    alt="example"
                    style={{ height: '30%', width: '30%' }}
                    src={
                      item.infos.split(`:`)[1] === 'Technical'
                        ? '../../../Technical_Skills.png'
                        : '../../../Soft_Skills.png'
                    }
                  />
                }
                style={
                  item.status === 'AWAITING' && item.referendum === 'false'
                    ? style1
                    : item.status === 'AWAITING' && item.referendum === 'true'
                      ? style2
                      : style3
                }
              >
                <List.Item key={item.name}>
                  <List.Item.Meta
                    title={<p>{item.name?.slice(0, 6) + '...' + item.name?.slice(-6, -1)}</p>}
                    description={
                      <div>
                        <p>Requested skill: {item.infos.split(':')[0]}</p>
                        <p>Request Status: {item.status}</p>
                        <p>Session is closed: {item.referendum}</p>
                      </div>
                    }
                  />
                </List.Item>
              </Card>
            )}
          />
        </div>
        <div>
          <p className=" flex gap-3">
            <Button
              onClick={() => {
                handleClick(true);
              }}
              disabled={voted || selected_proposal?.infos === ''}
              type="primary"
              className="bg-blue-600 text-white font-bold py-2 pb-10  text-xl"
            >
              AYES
            </Button>
            <Button
              onClick={() => {
                handleClick(false);
              }}
              disabled={voted || selected_proposal?.infos === ''}
              type="primary"
              className="bg-red-600 text-white font-bold py-2 pb-10   text-xl"
            >
              NAY
            </Button>
            <Button
              onClick={() => {
                handleClose();
              }}
              disabled={close}
              type="primary"
              className="bg-green-800 text-white font-bold py-2 pb-10   text-xl"
            >
              CLOSE
            </Button>

            <Card title={'User Information'} style={style4}>
              {out
                ? out.map((x) => (
                    <div>
                      <p className=" font-semibold">{x.split(':')[0]}:</p> {x.split(':')[1]}
                    </div>
                  ))
                : ''}
              <p className=" font-semibold">Voted:</p>
              {voted ? 'You voted!' : 'Not Yet!'}
            </Card>
          </p>
          <p>
            {!(showToast === false) ? (
              <Toast>
                <div
                  className={
                    'shadow-md rounded-md flex  text-white text-base items-center justify-normal ' +
                    (warning === true
                      ? ' bg-red-500 animate-bounce '
                      : ' bg-green-600  animate-pulse')
                  }
                >
                  <div>
                    {!(warning === true) ? (
                      <NotificationTwoTone twoToneColor="#52c41a" className="h-8 w-8" />
                    ) : (
                      <WarningTwoTone twoToneColor="#eb2f96" className="h-8 w-8" />
                    )}
                  </div>
                  <div className="p-2">{event}</div>
                  <Toast.Toggle
                    onClick={() => {
                      setShowToast(false);
                    }}
                  />
                </div>
              </Toast>
            ) : (
              <div className=" p-2"> </div>
            )}
          </p>
        </div>
      </div>
    );
  }
}
