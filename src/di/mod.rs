use crate::app::{
    command::create_user::{CreateUserCommand, UserWriteRepository},
    query::get_user::{GetUserQuery, UserRepository},
};

pub struct Container<R, Q>
where
    R: UserWriteRepository,
    Q: UserRepository,
{
    pub get_user_query: GetUserQuery<Q>,
    pub create_user_command: CreateUserCommand<R>,
}
impl<R, Q> Container<R, Q>
where
    R: UserWriteRepository,
    Q: UserRepository,
{
    pub fn new(repository: R, querier: Q) -> Self {
        let get_user_query = GetUserQuery::new(querier);
        let create_user_command = CreateUserCommand::new(repository);

        Self {
            get_user_query,
            create_user_command,
        }
    }
}
