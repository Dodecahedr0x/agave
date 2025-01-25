use {
    criterion::{criterion_group, criterion_main, Criterion, Throughput},
    solana_account::AccountSharedData,
    solana_pubkey::Pubkey,
    solana_svm::{
        account_overrides::AccountOverrides, account_overrides_dashmap::AccountOverrideDashMap,
    },
};

fn create_mock_account() -> AccountSharedData {
    AccountSharedData::new(1000, 100, &Pubkey::new_unique())
}

fn bench_hashmap(c: &mut Criterion) {
    c.benchmark_group("bench_account_overrides")
        .throughput(Throughput::Elements(1))
        .bench_function("hashmap 1x", |bencher| {
            bencher.iter(|| {
                let account = create_mock_account();
                let mut overrides = AccountOverrides::default();

                overrides.set_slot_history(Some(account));
            });
        })
        .bench_function("hashmap 100x", |bencher| {
            bencher.iter(|| {
                let mut overrides = AccountOverrides::default();

                for _ in 0..100 {
                    let account = create_mock_account();
                    overrides.set_slot_history(Some(account));
                }
            });
        })
        .bench_function("hashmap 10000x", |bencher| {
            bencher.iter(|| {
                let mut overrides = AccountOverrides::default();

                for _ in 0..10000 {
                    let account = create_mock_account();
                    overrides.set_slot_history(Some(account));
                }
            });
        })
        .bench_function("dashmap 1x", |bencher| {
            bencher.iter(|| {
                let account = create_mock_account();
                let mut overrides = AccountOverrideDashMap::default();

                overrides.set_slot_history(Some(account));
            });
        })
        .bench_function("dashmap 100x", |bencher| {
            bencher.iter(|| {
                let mut overrides = AccountOverrideDashMap::default();

                for _ in 0..100 {
                    let account = create_mock_account();
                    overrides.set_slot_history(Some(account));
                }
            });
        })
        .bench_function("dashmap 10000x", |bencher| {
            bencher.iter(|| {
                let mut overrides = AccountOverrideDashMap::default();

                for _ in 0..10000 {
                    let account = create_mock_account();
                    overrides.set_slot_history(Some(account));
                }
            });
        });
}

criterion_group!(benches, bench_hashmap,);
criterion_main!(benches);
