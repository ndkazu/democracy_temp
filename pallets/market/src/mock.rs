use super::*;
use crate as pallet_market;
use frame_support::{
	parameter_types,
	derive_impl,
	traits::{AsEnsureOriginWithArg, ConstU64, tokens::{PayFromAccount,UnityAssetBalanceConversion}},
	
};
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage, Perbill, Percent, Permill,
};
pub use frame_system::{EnsureRoot, EnsureSigned,EnsureWithSuccess,EnsureSignedBy};
use frame_system::{ EventRecord, Phase};

type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = AccountId32;
type Balance = u64;
pub type BlockNumber = u64;
pub type Stype = pallet_skills::SkillFamily;
pub type SLevel = pallet_skills::SkillLevel;
pub type Sapproval = pallet_skills::Approvals;


// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test 
	{
		System: frame_system,
		SkillsModule: pallet_skills,
		Balances: pallet_balances,
		MarketModule: pallet_market,
		Treasury: pallet_treasury,
		Bounties: pallet_bounties,
		Sudo:pallet_sudo,
		Collective: pallet_collective::<Instance1>,
		Assets: pallet_assets,
		AssetRate: pallet_asset_rate,
		Indices: pallet_indices,
		
	}
);

//helper types

fn default_max_proposal_weight() -> Weight {
	sp_runtime::Perbill::from_percent(50) * BlockWeights::get().max_block
}

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(Weight::MAX);
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = AccountId; // u64 is not enough to hold bytes used to generate bounty account
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u64>;
	type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
	pub const BasicWage: Balance = 50;
	pub const CheckPeriod: BlockNumber = 5;
	#[derive(Clone)]
	pub const MaxSkills: u32 = 128;
}
impl pallet_skills::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type StringLimit = ConstU32<256>;
	type BasicWage = BasicWage;
	type Currency = Balances;
	type CheckPeriod = CheckPeriod;
	type MaxSkills = MaxSkills;
	type CouncilOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
}


parameter_types! {
	pub const IndexDeposit: Balance = 1 ;
}
impl pallet_indices::Config for Test {
	type AccountIndex = u64;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

parameter_types! {
	pub const AssetDeposit: Balance = 100 ;
	pub const ApprovalDeposit: Balance = 1 ;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10 ;
	pub const MetadataDepositPerByte: Balance = 1 ;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU64<1>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo =();
	type RemoveItemsLimit = ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl pallet_asset_rate::Config for Test {
	type CreateOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type AssetKind = u32;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}


parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Test>;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeFreezeReason = ();
}

//---implementing pallet sudo---------
impl pallet_sudo::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo =();
}


parameter_types! {
	pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
	pub const BountyValueMinimum: Balance = 5 ;
	pub const BountyDepositBase: Balance = 1 ;
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub const CuratorDepositMin: Balance = 1 ;
	pub const CuratorDepositMax: Balance = 100 ;
	pub const BountyDepositPayoutDelay: BlockNumber = 1 ;
	pub const BountyUpdatePeriod: BlockNumber = 14;
}
impl pallet_bounties::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BountyDepositBase = ConstU64<80>;
	type BountyDepositPayoutDelay = ConstU64<3>;
	type BountyUpdatePeriod = ConstU64<2>;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMax = CuratorDepositMax;
	type CuratorDepositMin = CuratorDepositMin;
	type BountyValueMinimum = ConstU64<1>;
	type DataDepositPerByte = ConstU64<1>;
	type MaximumReasonLength = ConstU32<16384>;
	type WeightInfo = ();
	type ChildBountyManager = ();
}



parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaxApprovals: u32 = 100;
	pub TreasuryAccount: AccountId = Treasury::account_id();
	pub static SpendLimit: Balance = u64::MAX;
	pub static SpendLimit1: Balance = u64::MAX;
	
}


impl pallet_treasury::Config for Test {
	type Currency = pallet_balances::Pallet<Test>;
	type ApproveOrigin = frame_system::EnsureRoot<AccountId>;
	type RejectOrigin = frame_system::EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = ();
	type ProposalBond = ();
	type ProposalBondMinimum = ();
	type ProposalBondMaximum = ();
	type SpendPeriod = ConstU64<2>;
	type Burn = ();
	type BurnDestination = ();
	type PalletId = TreasuryPalletId;
	type SpendFunds = Bounties;
	type MaxApprovals = MaxApprovals;
	type WeightInfo = ();
	type SpendOrigin = frame_system::EnsureRootWithSuccess<Self::AccountId, SpendLimit>;
	type AssetKind = ();
	type Beneficiary = Self::AccountId;
	type BeneficiaryLookup = IdentityLookup<Self::AccountId>;
	type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
	type BalanceConverter =  UnityAssetBalanceConversion;
	type PayoutPeriod = ConstU64<2>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types!{
	//Every sp increase with an amount equal to xp_bonus, trigger 
	pub const xp_bonus: u32 = 1;
	pub const sp_trigger:u32 = 5;
	
}
impl pallet_market::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Sp = sp_trigger;
	type Xp = xp_bonus;
}


parameter_types! {
	pub const BackgroundMotionDuration: BlockNumber = 5;
	pub const BackgroundMaxProposals: u32 = 100;
	pub const BackgroundMaxMembers: u32 = 100;
	pub static MaxProposalWeight: Weight = default_max_proposal_weight();
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = BackgroundMotionDuration;
	type MaxProposals = BackgroundMaxProposals;
	type MaxMembers = BackgroundMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type MaxProposalWeight =MaxProposalWeight;
}

pub const ALICE: AccountId = AccountId::new([1u8; 32]);
pub const BOB: AccountId = AccountId::new([2u8; 32]);
pub const CHARLIE: AccountId = AccountId::new([3u8; 32]);
pub const RICHARD: AccountId = AccountId::new([4u8; 32]);
pub const DAVE: AccountId = AccountId::new([5u8; 32]);
pub const EVE: AccountId = AccountId::new([6u8; 32]);
pub const BSX: Balance = 100_000_000_000;

pub fn expect_events(e: Vec<RuntimeEvent>) {
	e.into_iter().for_each(frame_system::Pallet::<Test>::assert_has_event);
}

pub fn record(event: RuntimeEvent) -> EventRecord<RuntimeEvent, H256> {
	EventRecord { phase: Phase::Initialization, event, topics: vec![] }
}


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t= frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(ALICE, 200_000 * BSX),
			(BOB, 200_000 * BSX),
			(CHARLIE, 200_000 * BSX),
			(DAVE, 150_000 * BSX),
			(EVE, 150_000 * BSX),
			(Treasury::account_id(), 150_000_000 * BSX),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_collective::GenesisConfig::<Test, pallet_collective::Instance1> {
		members: vec![ALICE, BOB, CHARLIE],
		phantom: Default::default(),
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
}