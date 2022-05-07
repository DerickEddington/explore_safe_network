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
        RegisterAddress,
    },
    std::collections::BTreeMap,
    xor_name::XorName,
};


fn random_xorname() -> XorName
{
    XorName::random(&mut OsRng)
}


pub const TYPE_TAG: u64 = 0x079f8e78da75c9e7; // random generated

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
                Policy::Private(PrivatePolicy {
                    owner:       User::Key(client.public_key()),
                    permissions: BTreeMap::from_iter([(
                        User::Anyone,
                        PrivatePermissions::new(false, true),
                    )]),
                }),
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
        std::collections::{
            BTreeMap,
            BTreeSet,
        },
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
                Policy::Private(PrivatePolicy {
                    owner:       User::Key(owner_client.public_key()),
                    permissions: BTreeMap::from_iter([(
                        User::Anyone,
                        PrivatePermissions::new(false, true),
                    )]),
                }),
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
