#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
 

#[frame_support::pallet]
pub mod pallet {

	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
        use frame_support::inherent::Vec;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]

	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	
        #[pallet::storage]
        pub type CustomerName<T> = StorageValue<_, Vec<u8>, ValueQuery>;
        #[pallet::storage]
        pub type CustomerNo<T> = StorageValue<_, Vec<u8>, ValueQuery>;
        #[pallet::storage]
        pub type NoOfMembers<T> = StorageValue<_, u64, ValueQuery>;
        #[pallet::storage]
        pub type Source<T> = StorageValue<_, Vec<u8>, ValueQuery>;
        #[pallet::storage]
        pub type Destination<T> = StorageValue<_, Vec<u8>, ValueQuery>;
        #[pallet::storage]
        pub type StartDate<T> = StorageValue<_,Vec<u8>, ValueQuery >;
        #[pallet::storage]
        pub type EndDate<T> = StorageValue<_, Vec<u8>, ValueQuery>;
        #[pallet::storage]
        pub type Kilometer<T> = StorageValue<_, u32>;
        #[pallet::storage]
        pub type TravelsType<T> = StorageValue<_, Vec<u8>, ValueQuery>;
        #[pallet::storage]
        pub type Pktc<T> = StorageValue<_, u32, ValueQuery>;
        
        #[pallet::storage]
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	    pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	   #[pallet::event]
	   #[pallet::metadata(T::AccountId = "AccountId")]
	   #[pallet::generate_deposit(pub(super) fn deposit_event)]
	   pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		
                CustomerDetails(  Vec<u8>, Vec<u8>,u64, Vec<u8>,Vec<u8>,Vec<u8>,Vec<u8>,u32,Vec<u8>,u32,u32,T::AccountId),
               
              
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		

               #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn customer_details(origin: OriginFor<T>, name: Vec<u8>,no:Vec<u8>,nom:u64, source: Vec<u8>, destination: Vec<u8>,sdate: Vec<u8>,edate:Vec<u8>, Kilometer:u32,travelst:Vec<u8>,Pktc:u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;
                        let result = Kilometer * Pktc ;

                       <CustomerName<T>>::put(name.clone());
                       <CustomerNo<T>>::put(no.clone());
		        <NoOfMembers<T>>::put(nom.clone());
                       <Source<T>>::put(source.clone());
                       <Destination<T>>::put(destination.clone());
                       <StartDate<T>>::put(sdate.clone());
                       <EndDate<T>>::put(edate.clone());
                       <Kilometer<T>>::put(Kilometer.clone());
		       <TravelsType<T>>::put(travelst.clone());
                       <Pktc<T>>::put(Pktc.clone());
                       <Something<T>>::put(result);

                       
                     Self::deposit_event(Event::CustomerDetails( name,no,nom, source, destination, sdate, edate, Kilometer,travelst,Pktc,result ,sender ));
                       Ok(())
		}

           

		
	}
}


