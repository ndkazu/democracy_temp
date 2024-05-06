import React from 'react';
import AccountModal from './AccountModal';

export default function Header() {
  return (
    <div className="h-20 bg-gradient-to-r from-pink-700 to-blue-500 px-4 flex justify-between items-center">
      <div className=" font-bold text-4xl antialiased text-white">Skill Tracker</div>
      <AccountModal />
    </div>
  );
}
