use crate::mock::*;
use frame_support::assert_ok;
use sp_core::Blake2Hasher;
use sp_core::Hasher;

#[test]
fn archive_book_works() {
	new_test_ext().execute_with(|| {
		let title: Vec<u8> = "title".into();
		let author: Vec<u8> = "author".into();
		let url: Vec<u8> = "url".into();

		assert_ok!(Pallet::archive_book(
			RuntimeOrigin::signed(1),
			title.clone(),
			author.clone(),
			url.clone(),
		));

		let data = format!("{:?}{:?}", title, author);
		let hash = Blake2Hasher::hash(data.as_bytes());

		let stored_book_summary = Pallet::book_summary(hash).unwrap();
		assert_eq!(stored_book_summary.url, url);
	});
}
