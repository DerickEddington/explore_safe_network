use sn_api::{
    ContentType,
    Safe,
    XorUrl,
};

pub const TYPE_TAG: u64 = 0x079f8e78da75c9e7; // random generated

pub struct MBus
{
    safe:     Safe,
    register: XorUrl,
}

impl MBus
{
    pub async fn new(
        private: bool,
        safe: &Safe,
    ) -> Result<Self, Box<dyn std::error::Error>>
    {
        let name = None; // Let it generate a random one.
        Ok(Self {
            safe:     safe.clone(),
            register: safe.register_create(name, TYPE_TAG, private, ContentType::Raw).await?,
        })
    }
}


#[cfg(test)]
mod tests
{
    use {
        helpers::new_safe_instance,
        sn_api::SafeUrl,
    };

    #[tokio::test]
    async fn basis()
    {
        // let safe = Safe::dry_runner(Some(XorUrlBase::Base32z));
        let safe = new_safe_instance().await.unwrap();
        // let r = safe.register_create(None, 42, true, ContentType::Raw).await;

        let r = safe.multimap_create(None, 42, true).await;
        dbg!(&r);
        let m = r.unwrap();
        dbg!(SafeUrl::from_xorurl(&m).unwrap());

        let r = safe.multimap_insert(&m, (vec![0], vec![1]), Default::default()).await;
        dbg!(&r);
        let e = r.unwrap();

        let r = safe.multimap_get_by_hash(&m, e).await;
        dbg!(&r);

        let r = safe.multimap_get_by_key(&m, &[0]).await;
        dbg!(&r);
    }
}
