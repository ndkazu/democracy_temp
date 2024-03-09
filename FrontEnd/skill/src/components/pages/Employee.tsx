import React, { useEffect } from 'react';
import { useAppContext } from '../../contexts/AppContext';
import BN from 'bn.js';
import { NavLink } from 'react-router-dom';
import { toUnit } from '../shared/utils';
import { useAccountContext } from '../../contexts/AccountContext';

export default function Employee() {
  const { api, accounts, selectedAccount, selectedAddress, dispatch } = useAppContext();
  const {
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
}
