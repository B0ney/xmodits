mod loaders;
use loaders::{load_module, load_module_test, load_module_test_hash};

// cargo bench
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
// echo "const PATHS: &[&str] = &[" >> ./benches/list.txt; for i in $(ls tests/mods/*/*); do echo "   \"$i\"," >> list.txt; done; echo  "];" >> ./benches/list.txt;
const PATHS: &[&str] = &[
    "tests/mods/it/17_samples.it",
    "tests/mods/it/asikwp_-_fc-freedrive_chiptune.it",
    "tests/mods/it/before_the_explozion.it",
    "tests/mods/it/beyond_-_flute.it",
    "tests/mods/it/creagaia.it",
    "tests/mods/it/empty.it",
    "tests/mods/it/invalid.it",
    "tests/mods/it/no_samples.it",
    "tests/mods/it/sm-safari.it",
    "tests/mods/it/songofthesky.it",
    "tests/mods/mod/black_queen.mod",
    "tests/mods/mod/chop.mod",
    "tests/mods/mod/echobea3.mod",
    "tests/mods/mod/empty.mod",
    "tests/mods/mod/no_samples.mod",
    "tests/mods/mod/slash-kill-maim-hit.mod",
    "tests/mods/mod/sleep.mod",
    "tests/mods/mod/space_debris.mod",
    "tests/mods/mod/synthmat.mod",
    "tests/mods/s3m/arc-cell.s3m",
    "tests/mods/s3m/bluesky.s3m",
    "tests/mods/s3m/hip_-_640k_of_space.s3m",
    "tests/mods/s3m/invalid.s3m",
    "tests/mods/s3m/no_samples.s3m",
    "tests/mods/s3m/space_odyssey_v1_2.s3m",
    "tests/mods/s3m/synth_city.s3m",
    "tests/mods/s3m/torq_-_some_song.s3m",
    "tests/mods/xm/240-185_-_la_grenade_80s.xm",
    "tests/mods/xm/DEADLOCK.s3m",
    "tests/mods/xm/DEADLOCK.XM",
    "tests/mods/xm/invalid.xm",
    "tests/mods/xm/lovetrp.xm",
    "tests/mods/xm/no_samples.xm",
    "tests/mods/xm/sb-joint.xm",
    "tests/mods/xm/skuter_-_memoirs.xm",
    "tests/mods/xm/skuter_-_mind_validator.xm",
    "tests/mods/xm/sweetdre.xm",
    "tests/mods/xm/vagyakozas.xm",
    "tests/mods/xm/xo-sat.xm",
];

fn benchmark_modloader(c: &mut Criterion) {
    let ext = black_box(PATHS);

    fn test_load_mod(paths: &[&str]) {
        for ext in paths {
            load_module(ext);
        }
    }

    fn test_load_mod_phf(paths: &[&str]) {
        for ext in paths {
            load_module_test(ext);
        }
    }

    fn test_load_mod_hash(paths: &[&str]) {
        for ext in paths {
            load_module_test_hash(ext);
        }
    }

    let mut group = c.benchmark_group("compare");

    group.bench_function("Match + lazy loaders",|f| f.iter(|| test_load_mod(ext)));
    group.bench_function("Phf",|f| f.iter(|| test_load_mod_phf(ext)));
    group.bench_function("unordered Hash",|f| f.iter(|| test_load_mod_hash(ext)));

    group.finish();
}

criterion_group!(benches, benchmark_modloader);
criterion_main!(benches);