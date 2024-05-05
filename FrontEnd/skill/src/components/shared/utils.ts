import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';
import { BN, formatBalance } from '@polkadot/util';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
export function toUnit(balance: string, decimals: number) {
  formatBalance.setDefaults({ decimals: 11, unit: 'USD' });
  let value = formatBalance(new BN(balance), { withSi: true, withZero: false });
  return value;
}
