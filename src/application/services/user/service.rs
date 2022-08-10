use crate::domain::user::value_objects::{UserID, UserIDProvider};
use crate::domain::user::{entity::User, repository::Repository};
use crate::error::Result;

// pub struct UserService<'a, R, P>
// where
//     R: Repository<'a>,
//     P: UserIDProvider,
// {
//     pub repository: R,
//     pub id_provider: P,
// }

// impl<R, P> UserService<R, P>
// where
//     for<'a> R: Repository + Store<'a>,
//     P: UserIDProvider,
// {
//     pub fn new(repository: R, id_provider: P) -> Self {
//         Self {
//             repository,
//             id_provider,
//         }
//     }

//     pub async fn get_by_id(&self, id: &UserID) -> Result<User> {
//         self.repository.get_by_id(id).await
//     }
// }
