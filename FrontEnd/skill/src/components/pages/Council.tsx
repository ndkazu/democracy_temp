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
    dispatch2,
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
        dispatch2({ type: `SET_SELECTED_PROPOSAL`, payload: prop });
      }
      console.log(prop.infos);
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
  }

  function getDatas() {
    if (!api || !selectedAccount) return;
    let tdata: DataType[] = [];
    let props: Proposal[] = [];

    api.query.skillsModule.skillsProposalList.entries((all: any[]) => {
      // if (!council_members.includes(selectedAccount)) return;
      all.forEach(([key, value]) => {
        let Prop = value.toHuman();
        if (!Prop) return;
        let acc0 = Prop.account;
        let acc1 = accounts.find((account) => account.address === acc0);
        if (!acc1) return;

        //console.log(`names are:${acc1.meta.name}`)
        let status = Prop.approved.toString();
        let referendum = Prop.sessionClosed.toString();
        let hash = Prop.proposalHash.toString();
        let skill = Prop.skill;
        let infos = `${skill.metadata}:${skill.skillType}:${skill.skillLevel}:${skill.confirmed}:${skill.skillNumber}`;

        let dtype: DataType = {
          name: acc1.meta.name,
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

      dispatch2({ type: `SET_DATAS`, payload: tdata });
      dispatch2({ type: `SET_PROPOSALS`, payload: props });
    });
  }

  return <div>Council Page</div>;
}
