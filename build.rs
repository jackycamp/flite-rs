use std::path::PathBuf;
use std::fs;

extern crate cc;

fn get_c_files_in_dir_with_exclude(dir: &str, exclude: Option<&[&str]> ) -> Vec<PathBuf> {
    let mut cfiles = Vec::new();
    let dir = PathBuf::from(dir);

    fs::read_dir(dir.clone()).expect(&format!("can't read dir {:?}", dir));

    let exclude_paths: Vec<PathBuf> = exclude
        .unwrap_or(&[])
        .iter()
        .map(PathBuf::from)
        .collect();

    for item in fs::read_dir(dir).unwrap() {
        println!("item {item:?}");
        let item = item.unwrap();
        let item_path = item.path();
        if item_path.is_dir() || item_path.extension().is_none() {
            continue;
        }
        if exclude_paths.iter().any(|exclude_path| item_path.ends_with(exclude_path)) {
            continue;
        }
        if item_path.extension().unwrap() == "c" {
            println!("{item_path:?}");
            cfiles.push(item_path);
        }
    }
    cfiles
}

fn get_c_files_in_dir(dir: &str) -> Vec<PathBuf> {
    get_c_files_in_dir_with_exclude(dir, None)
}


fn main() {
    cc::Build::new()
        .define("CST_NO_SOCKETS", None)
        .files(get_c_files_in_dir("extern/flite/src/synth"))
        .files(get_c_files_in_dir_with_exclude("extern/flite/src/utils", Some(&["cst_mmap_win32.c", "cst_file_palmos.c", "cst_file_wince.c"])))
        .files(get_c_files_in_dir("extern/flite/src/hrg"))
        .files(get_c_files_in_dir("extern/flite/src/cg"))
        .files(get_c_files_in_dir("extern/flite/src/lexicon"))
        .files(get_c_files_in_dir("extern/flite/src/regex"))
        .files(get_c_files_in_dir("extern/flite/src/speech"))
        .files(get_c_files_in_dir("extern/flite/src/stats"))
        .files(get_c_files_in_dir("extern/flite/src/wavesynth"))
        .files(get_c_files_in_dir("extern/flite/lang/cmu_us_slt"))
        // .files(get_c_files_in_dir("extern/flite/lang/cmu_us_kal"))
        .files(get_c_files_in_dir("extern/flite/lang/usenglish"))
        .files(get_c_files_in_dir_with_exclude("extern/flite/lang/cmulex", Some(&["cmu_lex_phones_huff_table.c", "cmu_lex_num_bytes.c", "cmu_lex_data_raw.c", "cmu_lex_entries_huff_table.c"])))
        .files(get_c_files_in_dir_with_exclude("extern/flite/src/audio", Some(&["au_oss.c", "au_alsa.c", "au_wince.c", "au_pulseaudio.c", "au_win.c", "au_palmos.c", "au_sun.c"])))
        .include("extern/flite/include")
        .include("extern/flite/src/cg")
        .include("extern/flite/lang/usenglish")
        .include("extern/flite/lang/cmulex")
        .include("extern/flite/lang/cmu_us_slt")
        .include("extern/flite/lang/cmu_us_kal")
        .compile("flite")
}
