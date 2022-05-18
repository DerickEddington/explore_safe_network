use {
    rand::rngs::OsRng,
    sn_client::Client,
    sn_interface::types::{
        register::{
            Policy,
            PrivatePermissions,
            PrivatePolicy,
            User,
        },
        PublicKey,
        RegisterAddress,
    },
    std::collections::BTreeMap,
    xor_name::XorName,
};


fn random_xorname() -> XorName
{
    XorName::random(&mut OsRng)
}

fn write_only_private_policy(
    owner: PublicKey,
    users: impl IntoIterator<Item = User>,
) -> Policy
{
    let write_only_perm = PrivatePermissions::new(false, true);
    PrivatePolicy {
        owner:       User::Key(owner),
        permissions: BTreeMap::from_iter(users.into_iter().map(|user| (user, write_only_perm))),
    }
    .into()
}


pub const TYPE_TAG: u64 = 0x079f8e78da75c9e7; // random generated

#[allow(dead_code)] // TODO
pub struct MBus
{
    client:   Client,
    register: RegisterAddress,
}

impl MBus
{
    pub async fn new(client: &Client) -> Result<Self, Box<dyn std::error::Error>>
    {
        let (register, ops_that_create) = client
            .create_register(
                random_xorname(),
                TYPE_TAG,
                write_only_private_policy(client.public_key(), [User::Anyone]),
            )
            .await?;

        client.publish_register_ops(ops_that_create).await?;

        Ok(Self { client: client.clone(), register })
    }
}


#[cfg(test)]
mod tests
{
    use {
        super::*,
        helpers::connect_to_testnet,
        std::collections::BTreeSet,
    };

    #[tokio::test]
    async fn basis()
    {
        let owner_client = connect_to_testnet().await.unwrap();

        let reg_name = random_xorname();
        let r = owner_client
            .create_register(
                reg_name,
                42,
                write_only_private_policy(owner_client.public_key(), [User::Anyone]),
            )
            .await
            .unwrap();
        dbg!(&r);
        let (reg_addr, ops_that_create) = r;
        owner_client.publish_register_ops(ops_that_create).await.unwrap();

        let r = owner_client
            .write_to_register(reg_addr, vec![1, 2, 3], BTreeSet::new())
            .await
            .unwrap();
        dbg!(&r);
        let (_entry_hash, ops_that_write) = r;
        owner_client.publish_register_ops(ops_that_write).await.unwrap();

        let r = owner_client.read_register(reg_addr).await.unwrap();
        dbg!(&r);


        let stranger_client = connect_to_testnet().await.unwrap();

        let r = stranger_client
            .write_to_register(reg_addr, vec![9, 8, 7], BTreeSet::new())
            .await
            .unwrap();
        dbg!(&r);
        let (_entry_hash, ops_that_write) = r;
        stranger_client.publish_register_ops(ops_that_write).await.unwrap();

        let r = stranger_client.read_register(reg_addr).await.unwrap();
        dbg!(&r);
    }
}
