//! # Pallet Archiver
//!
//! ## Overview
//!
//! This pallet allows users to create an archive record for a book.
//! Only one record can be created for a specific book
//!
//! ## Interface
//!
//! ### Config
//!
//! ### Dispatchable functions
//!
//! * `archive_book(orgin, title, author, url, archiver, timestamp)` - Archive a specified book
//!
//! ### RPC query endpoints
//!
//! * `book_summary( hash(title + author) )` - Retrieve book summary from the archive

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::Hash;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::format;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a book is archived
		BookArchived { who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Book already exist in archive
		BookAlreadyExistInArchive,
	}

	/// Book summary
	#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub struct BookSummary<AccountId, BlockNumber> {
		pub title: Vec<u8>,     // title of book
		pub author: Vec<u8>,    // author of book
		pub url: Vec<u8>,       // web url to off-chain storage
		archiver: AccountId,    // account id of archiver
		timestamp: BlockNumber, // time when book was archived
	}

	/// Archive storage
	///
	/// Maps a book hash to book summary
	/// Book hash is Blake2 hash of book title and author
	#[pallet::storage]
	#[pallet::getter(fn book_summary)]
	pub(super) type ArchiveStore<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash,
		BookSummary<T::AccountId, T::BlockNumber>,
		OptionQuery,
	>;

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(1)]
		#[pallet::weight(100_000_000)]
		pub fn archive_book(
			origin: OriginFor<T>,
			title: Vec<u8>,
			author: Vec<u8>,
			url: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let signer = ensure_signed(origin)?;

			let title = title.to_ascii_lowercase();
			let author = author.to_ascii_lowercase();

			// Create book pre-signature
			let pre_image = format!("{:?}{:?}", title, author,);

			// Get book hash
			let book_hash = T::Hashing::hash(&pre_image.as_bytes());

			// Verify that title and author have not already been stored
			ensure!(
				!ArchiveStore::<T>::contains_key(&book_hash),
				Error::<T>::BookAlreadyExistInArchive
			);

			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Create specified book summary
			let book_summary = BookSummary {
				title,
				author,
				url,
				archiver: signer.clone(),
				timestamp: current_block,
			};

			// Store book summary in archive
			ArchiveStore::<T>::insert(&book_hash, book_summary);

			// Emit an event that the book was archived.
			Self::deposit_event(Event::BookArchived { who: signer });

			Ok(())
		}
	}
}
