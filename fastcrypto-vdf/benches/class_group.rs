// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkGroup, Criterion};
use fastcrypto::groups::Doubling;
use fastcrypto_vdf::class_group::discriminant::Discriminant;
use fastcrypto_vdf::class_group::QuadraticForm;
use fastcrypto_vdf::ParameterizedGroupElement;
use num_bigint::BigInt;
use num_traits::Num;
use rand::{thread_rng, RngCore};

fn class_group_ops_single<M: Measurement>(
    discriminant_string: &str,
    group: &mut BenchmarkGroup<M>,
) {
    let discriminant =
        Discriminant::try_from(BigInt::from_str_radix(discriminant_string, 10).unwrap()).unwrap();
    let discriminant_size = discriminant.bits();
    let x = QuadraticForm::generator(&discriminant).mul(&BigInt::from(1234));
    let y = QuadraticForm::generator(&discriminant).mul(&BigInt::from(4321));
    let z = y.clone();

    group.bench_function(format!("Compose/{}", discriminant_size), move |b| {
        b.iter(|| x.compose(&y))
    });

    group.bench_function(format!("Double/{}", discriminant_size), move |b| {
        b.iter_batched(|| z.clone(), |z| z.double(), BatchSize::SmallInput)
    });
}

fn class_group_ops(c: &mut Criterion) {
    let mut group: BenchmarkGroup<_> = c.benchmark_group("Class Group".to_string());

    class_group_ops_single("-177936553897922261333164712410242884021141613334565149505848952826212491241771489747671651876796162246463307642229416178115038439759411468976306741727054586407871065937781422970533238726218439981299971510624199735799141406458705159532021234549127390010928558972193365915987838285442951665356778289757172235943", &mut group);
    class_group_ops_single("-23847247576959809407620518371643827142851600013982827283452201808476362679737007351478256611970011299818463606695939850757804842744423478520579143718664008954313076284985803848944695073539132381339505840123539160487844863469984182420796118328898791241477525662002452306976347345400556627774302685021609312012814515255952971689521318993322456321786703461532401532357072300154368747634872702010117806469012662322855405602958496469070700447595558569638530268626808054913729109827208897181454138769346082237168707092766224594001630621792974930286552997453103372073081585433844131063808863644588266240224171853046660887399", & mut group);
    class_group_ops_single("-197094279717529776652945533421408519016291293185778176422038767173246838389717778782272450609952179792102389097362657787152898007436991089430517979761145200893975140029279440383697629952398509684430189989830512427761221044255503309237697000446508821686655886069366603792908696660367648281136978401042076354619587515552611650395121072487799107192700364331538210709886133279169829259881605487142555274403314509719321602412760314496712012939372327177464352472192738122541539747842405435171078768578664156285412471750348778431888800482596404122201686947621151032470989798594881908508768154982514267787085456831726879055929531619461354230569362180363281846948763424056650300352728927552479847814231289623672826128091486169286759", & mut group);
    class_group_ops_single("-3956718340719431033560816005739172412770466038703883350122595604635576709778731043309649272634605425735063624961596232735523376358742586480084965522907889249504047757258883253188259997112353246294323895993621766064597867526555590598296171109353515435289599237581716817331650248439511065683812661746851260538223197043808247010651962962398117206670503394901226393544809521031397039151671384417895714291888633743136733122871000628264376743806741659217599861141099968091237561343232177937280916663464976799422526037223295835103220909629798144507776992742385829474209304917863091971426479552645067278361106093545925188309289850090156462919761399169738282697646499095260815062205117198915610910901974886408275636330747461116245431578813689565691787676837733109337359377237752443898143986060895378572144245069588193342265623936118710486379006798704589510096698445426370143184307451927004120706539519891935325715903611926844068327127", & mut group);
}

fn qf_from_seed_single<M: Measurement>(discriminant_string: &str, group: &mut BenchmarkGroup<M>) {
    let discriminant =
        Discriminant::try_from(BigInt::from_str_radix(discriminant_string, 10).unwrap()).unwrap();

    for k in [1, 2, 4, 8, 16, 26, 32, 64] {
        if k > fastcrypto_vdf::class_group::hash::largest_allowed_k(&discriminant) {
            continue;
        }

        let bits = discriminant.bits();
        group.bench_function(format!("{} bits/{}", bits, k), |b| {
            let mut seed = [0u8; 32];
            thread_rng().fill_bytes(&mut seed);
            b.iter(|| QuadraticForm::hash_to_group(&seed, &discriminant, k))
        });
    }
}

fn qf_from_seed(c: &mut Criterion) {
    let mut group: BenchmarkGroup<_> = c.benchmark_group("QF from seed".to_string());
    qf_from_seed_single("-177936553897922261333164712410242884021141613334565149505848952826212491241771489747671651876796162246463307642229416178115038439759411468976306741727054586407871065937781422970533238726218439981299971510624199735799141406458705159532021234549127390010928558972193365915987838285442951665356778289757172235943", & mut group);
    qf_from_seed_single("-23847247576959809407620518371643827142851600013982827283452201808476362679737007351478256611970011299818463606695939850757804842744423478520579143718664008954313076284985803848944695073539132381339505840123539160487844863469984182420796118328898791241477525662002452306976347345400556627774302685021609312012814515255952971689521318993322456321786703461532401532357072300154368747634872702010117806469012662322855405602958496469070700447595558569638530268626808054913729109827208897181454138769346082237168707092766224594001630621792974930286552997453103372073081585433844131063808863644588266240224171853046660887399", & mut group);
    qf_from_seed_single("-197094279717529776652945533421408519016291293185778176422038767173246838389717778782272450609952179792102389097362657787152898007436991089430517979761145200893975140029279440383697629952398509684430189989830512427761221044255503309237697000446508821686655886069366603792908696660367648281136978401042076354619587515552611650395121072487799107192700364331538210709886133279169829259881605487142555274403314509719321602412760314496712012939372327177464352472192738122541539747842405435171078768578664156285412471750348778431888800482596404122201686947621151032470989798594881908508768154982514267787085456831726879055929531619461354230569362180363281846948763424056650300352728927552479847814231289623672826128091486169286759", & mut group);
    qf_from_seed_single("-3956718340719431033560816005739172412770466038703883350122595604635576709778731043309649272634605425735063624961596232735523376358742586480084965522907889249504047757258883253188259997112353246294323895993621766064597867526555590598296171109353515435289599237581716817331650248439511065683812661746851260538223197043808247010651962962398117206670503394901226393544809521031397039151671384417895714291888633743136733122871000628264376743806741659217599861141099968091237561343232177937280916663464976799422526037223295835103220909629798144507776992742385829474209304917863091971426479552645067278361106093545925188309289850090156462919761399169738282697646499095260815062205117198915610910901974886408275636330747461116245431578813689565691787676837733109337359377237752443898143986060895378572144245069588193342265623936118710486379006798704589510096698445426370143184307451927004120706539519891935325715903611926844068327127", & mut group);
}

criterion_group! {
    name = class_group_benchmarks;
    config = Criterion::default().sample_size(100);
    targets = class_group_ops, qf_from_seed
}

criterion_main!(class_group_benchmarks);
