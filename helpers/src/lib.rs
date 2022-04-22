// Re-exporting from `sn_api::test_helpers` requires my modified `expose--test_helpers` branch of
// `sn_api`, which is not ideal, but is quick and easy.  Instead, it seems like the desired
// functions could be re-implemented here to provide them, but that would be more involved.

pub use sn_api::test_helpers::{
    new_read_only_safe_instance,
    new_safe_instance,
};

#[cfg(test)]
mod tests
{
    use super::*;

    #[tokio::test]
    async fn it_works()
    {
        let _a = new_safe_instance().await.unwrap();
        let _b = new_read_only_safe_instance().await.unwrap();
    }
}
